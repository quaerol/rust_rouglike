pub use map::*;
use rltk::{GameState, Point, Rltk, RGB};
use serde::*;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use tutorial::*; // use tutorial lib
fn main() -> rltk::BError {
    // use builder
    use rltk::RltkBuilder;

    // initialise context
    let mut context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // 地图显示复古的感觉
    context.with_post_scanlines(true);

    // create world,because entity will add in world,so world is mutable
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::PreRun,
    };

    // -------------register component--------------------------------
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    // gs.ecs.register::<LeftWalker>();
    // gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    gs.ecs.register::<Viewshed>(); // 将组件注册到系统中
    gs.ecs.register::<BlocksTile>();

    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Confusion>();

    // 物品组件
    gs.ecs.register::<Item>();

    gs.ecs.register::<InBackpack>();

    // 意图组件
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<WantsToUseItem>();
    gs.ecs.register::<WantsToDropItem>();

    // 标记组件
    gs.ecs.register::<SimpleMarker<SerializeMe>>();

    // 插入一个实体标记 作为资源
    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
    // ------------------create entity 创建实体 ----------------------------------------------------
    // 为等级 1 创建地图
    let map = Map::new_map_rooms_and_corridors(1);

    // 使用 spawner 创建玩家 怪物 物品
    // 玩家的位置在第一个房间的中心位置
    let (player_x, player_y) = map.rooms[0].center();
    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    // ----------------------房间 怪物 物品 生成器代码------------------------
    // 在创建房间后同时创建怪物 和 物品，两者的位置有房间确定
    // 随机数 生成器 作为一种 资源 随机创建
    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room);
    }

    // --------------add resource in world  ,shared data the whole ecs can use --------------------------------
    // 将map 插到world 中
    gs.ecs.insert(map);

    // 将玩家的位置作为 资源 插入 ecs 中 Point 是表示玩家位置的资源
    gs.ecs.insert(Point::new(player_x, player_y));

    // 将玩家实体转为资源，这样可以全局使用
    gs.ecs.insert(player_entity);

    // 将运行状态作为资源插入进世界中
    gs.ecs.insert(RunState::PreRun);

    // 插入日志 作为资源
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Welcome to Rusty Roguelike".to_string()],
    });
    // ------------------------game mian loop------------------------
    rltk::main_loop(context, gs)
}
