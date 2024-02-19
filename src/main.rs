pub use map::*;
use rltk::{GameState, Point, Rltk, RGB};
use serde::*;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use tutorial::*; // use tutorial lib
const SHOW_MAPGEN_VISUALIZER : bool = true;
fn main() -> rltk::BError {
    // use builder
    use rltk::RltkBuilder;

    // initialise context
    let mut context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // 地图显示复古的感觉
    context.with_post_scanlines(true);

    // 游戏状态，包含游戏世界和游戏的运行状态
    let mut gs = State {
        ecs: World::new(),
        mapgen_next_state : Some(RunState::MainMenu{ menu_selection: gui::MainMenuSelection::NewGame }),
        mapgen_index : 0,
        mapgen_history: Vec::new(),
        mapgen_timer: 0.0
    };

    // 向ecs 中的 world 注册组件
    // -------------register component--------------------------------
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();

    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    gs.ecs.register::<Viewshed>(); // 将组件注册到系统中
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<Confusion>();

    // 物品组件
    gs.ecs.register::<Item>();
    gs.ecs.register::<ProvidesHealing>();
    gs.ecs.register::<InflictsDamage>();
    gs.ecs.register::<AreaOfEffect>();
    gs.ecs.register::<Consumable>();
    gs.ecs.register::<Ranged>();
    gs.ecs.register::<InBackpack>();

    // 装备
    gs.ecs.register::<Equippable>();
    gs.ecs.register::<Equipped>();
    gs.ecs.register::<MeleePowerBonus>();
    gs.ecs.register::<DefenseBonus>();
    gs.ecs.register::<WantsToRemoveItem>();

    // 意图组件
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<WantsToUseItem>();
    gs.ecs.register::<WantsToDropItem>();

    // 标记组件
    gs.ecs.register::<SimpleMarker<SerializeMe>>();

    // 插入一个实体标记 作为资源
    gs.ecs.register::<SerializationHelper>();
    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    // 粒子生命周期组件
    gs.ecs.register::<ParticleLifetime>();
    // 将ParticleBuilder 作为资源
    gs.ecs.insert(particle_system::ParticleBuilder::new());

    // hunger food
    gs.ecs.register::<HungerClock>();
    gs.ecs.register::<ProvidesFood>();

    // magic mapping
    gs.ecs.register::<MagicMapper>();

    // trap
    gs.ecs.register::<Hidden>();
    gs.ecs.register::<EntryTrigger>();
    gs.ecs.register::<EntityMoved>();
    gs.ecs.register::<SingleActivation>();

    // ------------------create entity 创建实体 ----------------------------------------------------
    // level 1 创建地图
    let mut builder = map_builders::random_builder(1);
    builder.build_map();
    let player_start = builder.get_starting_position();
    let map = builder.get_map();
    // 使用 spawner 创建玩家 怪物 物品
    // 玩家的位置在第一个房间的中心位置y
    let (player_x, player_y) = (player_start.x, player_start.y);
    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    // ----------------------房间 怪物 物品 生成器代码------------------------
    // 在创建房间后同时创建怪物 和 物品，两者的位置有房间确定
    // 随机数 生成器 作为一种 资源 随机创建
    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    builder.spawn_entities(&mut gs.ecs);

    // --------------add resource in world  ,shared data the whole ecs can use --------------------------------
    // 将map 插到world 中
    gs.ecs.insert(map);

    // 将玩家的位置作为 资源 插入 ecs 中 Point 是表示玩家位置的资源
    gs.ecs.insert(Point::new(player_x, player_y));

    // 将玩家实体转为资源，这样可以全局使用
    gs.ecs.insert(player_entity);

    // 将运行状态作为资源插入进世界中
    gs.ecs.insert(RunState::MapGeneration{} );

    // 插入日志 作为资源
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Welcome to Rusty Roguelike".to_string()],
    });

    //  insert the dungeon graphic into Specs as a resource so we can access our sprites anywhere
    gs.ecs.insert(rex_assets::RexAssets::new());
    // ------------------------game mian loop------------------------
    rltk::main_loop(context, gs)
}
