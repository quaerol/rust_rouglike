use std::{fs, path::Path};

use super::components::*;
use crate::SerializeMe;
// use specs::error::Infallible;
use specs::prelude::*;
use specs::saveload::{
    DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker, SimpleMarkerAllocator,
};
use specs::World;
use std::convert::Infallible;

use std::fs::File;

// serialize_individually 宏解决 Serde 和 Specs 之间进行协同工作的问题，如何将Specs 中的Component　类型序列化
macro_rules! serialize_individually {
    // 宏的参数 ，data 是一个元组
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<Infallible, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

// 反序列化， 将json 文件中的数据，反序列化为Serde type, ranh Specs 的Component 类型
macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
        $(
        DeserializeComponents::<Infallible, _>::deserialize(
            // 参数都是引用
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &mut $data.0, // entities
            &mut $data.1, // marker
            &mut $data.2, // allocater
            &mut $de,
        )
        .unwrap();
        )*
    };
}
// 仅在非 Web 汇编平台上时编译
#[cfg(not(target_arch = "wasm32"))]
pub fn save_game(ecs: &mut World) {
    // Create helper
    // 复制一份地图
    let mapcopy = ecs.get_mut::<super::map::Map>().unwrap().clone();
    let savehelper = ecs
        .create_entity()
        .with(SerializationHelper { map: mapcopy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    // Actually serialize
    {
        let data = (
            ecs.entities(),
            ecs.read_storage::<SimpleMarker<SerializeMe>>(),
        );
        // 序列化的结果保存在 savegame.json 文件中
        let writer = File::create("./savegame.json").unwrap();
        let mut serializer = serde_json::Serializer::new(writer);
        // 使用宏，将Specs 中的组件序列化
        serialize_individually!(
            ecs,
            serializer,
            data,
            Position,
            Renderable,
            Player,
            Viewshed,
            Monster,
            Name,
            BlocksTile,
            CombatStats,
            SufferDamage,
            WantsToMelee,
            Item,
            Consumable,
            Ranged,
            InflictsDamage,
            AreaOfEffect,
            Confusion,
            ProvidesHealing,
            InBackpack,
            WantsToPickupItem,
            WantsToUseItem,
            WantsToDropItem,
            SerializationHelper,
            Equippable,
            Equipped,
            MeleePowerBonus,
            DefenseBonus,
            WantsToRemoveItem,
            ParticleLifetime
        );
    }
    // Clean up
    ecs.delete_entity(savehelper).expect("Crash on cleanup");
}
// 一个语言添加新的语法，编译器完成这个新得语法对应得特性
// 添加一个存根函数，stub function save_game, 这个存根函数会在web 平台时 进行编译，这个函数没有实现，
// 没有这个函数 web 平台得编译会失败
#[cfg(target_arch = "wasm32")]
pub fn save_game(_ecs: &mut World) {}
// 是否存在被保存的游戏状态
pub fn does_save_exist() -> bool {
    // 游戏状态文件是否存在
    Path::new("./savegame.json").exists()
}
// 删除 游戏存档
pub fn delete_save() {
    if Path::new("./savegame.json").exists() {
        std::fs::remove_file("./savegame.json").expect("Unable to delete file");
    }
}
pub fn load_game(ecs: &mut World) {
    {
        // delete everything, 先将所有的实体保存在一个Vector 中，然后遍历这个Vector 删除其中的每一个实体
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Deletion failed");
        }

        // 从 文件中读取数据
        let data = fs::read_to_string("./savegame.json").unwrap();
        // 反序列化
        let mut de = serde_json::Deserializer::from_str(&data);

        {
            // 为宏构建元组，这需要对实体存储的可变访问、对标记存储的写入访问和分配器（来自规范）。
            let mut d = (
                &mut ecs.entities(),
                &mut ecs.write_storage::<SimpleMarker<SerializeMe>>(),
                &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>(),
            );

            deserialize_individually!(
                ecs,
                de,
                d,
                Position,
                Renderable,
                Player,
                Viewshed,
                Monster,
                Name,
                BlocksTile,
                CombatStats,
                SufferDamage,
                WantsToMelee,
                Item,
                Consumable,
                Ranged,
                InflictsDamage,
                AreaOfEffect,
                Confusion,
                ProvidesHealing,
                InBackpack,
                WantsToPickupItem,
                WantsToUseItem,
                WantsToDropItem,
                SerializationHelper,
                Equippable,
                Equipped,
                MeleePowerBonus,
                DefenseBonus,
                WantsToRemoveItem,
                ParticleLifetime
            );
        }

        let mut deleteme: Option<Entity> = None;

        // another block, aviod borrow conflicts with the previous code and the entity deletion
        {
            let entities = ecs.entities();
            let helper = ecs.read_storage::<SerializationHelper>();

            let player = ecs.read_storage::<Player>();
            let position = ecs.read_storage::<Position>();
            // first iterate all entities with a SerializationHelper type, if find helper entity
            for (e, h) in (&entities, &helper).join() {
                // get access to the resource storing the map - and replace it with the helper map
                let mut worldmap = ecs.write_resource::<super::map::Map>();
                *worldmap = h.map.clone();
                // since we are not serializing tile_content, replace it with an empty set of vectors
                worldmap.tile_content = vec![Vec::new(); super::map::MAPCOUNT];

                deleteme = Some(e);
            }

            // then find the player, by iterating entities with a Player type anf a Position type,
            for (e, _p, pos) in (&entities, &player, &position).join() {
                // store the world resource for player entity and his/her position
                let mut ppos = ecs.write_resource::<rltk::Point>();
                *ppos = rltk::Point::new(pos.x, pos.y);
                let mut player_resource = ecs.write_resource::<Entity>();
                *player_resource = e;
            }
        }
        //finally, we delete the helper entity - so we won't have a duplicate entity if we save the game again
        ecs.delete_entity(deleteme.unwrap())
            .expect("Unable to delete helper");
    }
}
