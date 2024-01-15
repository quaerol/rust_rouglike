use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

pub use map::*;
use tutorial::*; // use tutorial lib
fn main() -> rltk::BError {
    // use builder
    use rltk::RltkBuilder;

    // initialise context
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()
        .unwrap();

    // create world,because entity will add in world,so world is mutable
    let mut gs = State { ecs: World::new() };

    // -------------register component
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    // gs.ecs.register::<LeftWalker>();
    // gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>(); // 将组件注册到系统中

    // --------------add source in world  ,shared data the whole ecs can use
    let map = Map::new_map_rooms_and_corridors();
    // 玩家的位置在第一个房间的中心位置
    let (player_x, player_y) = map.rooms[0].center();
    // 将map 插到world 中
    gs.ecs.insert(map);
    // ------------------create entity
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('o'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            // 为玩家添加视野组件
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    // for i in 0..10 {
    //     gs.ecs
    //         .create_entity()
    //         .with(Position { x: i * 7, y: 20 }) //玩家的位置是第一个rooms的中心位置
    //         .with(Renderable {
    //             glyph: rltk::to_cp437('x'),
    //             fg: RGB::named(rltk::YELLOW),
    //             bg: RGB::named(rltk::BLACK),
    //         })
    //         .with(LeftMover {}) // LeftMover 标记需要移动的
    //         .build();
    // }

    // game mian loop
    rltk::main_loop(context, gs)
}
