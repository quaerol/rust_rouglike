use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;

pub use map::*;
use tutorial::*; // use tutorial lib
fn main() -> rltk::BError {
    // use builder
    use rltk::RltkBuilder;

    // initialise context
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // create world,because entity will add in world,so world is mutable
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
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

    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();
    gs.ecs.register::<InBackpack>();

    // ------------------create entity 创建实体 ----------------------------------------------------

    // 使用 spawner 创建玩家 怪物 物品
    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    // ----------------------房间 怪物 物品 生成器代码------------------------
    // 在创建房间后同时创建怪物 和 物品，两者的位置有房间确定
    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room);
    }

    // --------------add resource in world  ,shared data the whole ecs can use --------------------------------
    // 将map 插到world 中
    let map = Map::new_map_rooms_and_corridors();
    gs.ecs.insert(map);

    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    // 玩家的位置在第一个房间的中心位置
    let (player_x, player_y) = map.rooms[0].center();

    // 将玩家的位置作为 资源 插入 ecs 中 Point 是表示玩家位置的资源
    gs.ecs.insert(Point::new(player_x, player_y));

    // 将玩家实体转为资源，这样可以全局使用
    gs.ecs.insert(player_entity);

    // ------------------------game mian loop------------------------
    rltk::main_loop(context, gs)
}
