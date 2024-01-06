use rltk::RGB;
use specs::prelude::*;

use tutorial::*; // use tutorial li/b

fn main() {
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
    gs.ecs.register::<LeftWalker>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    // --------------add source in world  ,shared data the whole ecs can use
    let (rooms, map) = new_map_rooms_and_corridors();
    // 将map 插到world 中
    gs.ecs.insert(map);

    // 玩家的位置在第一个房间的中心位置
    let (player_x, player_y) = rooms[0].center();

    // ------------------create entity
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('o'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('x'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {}) // LeftMover 标记需要移动的
            .build();
    }

    // game mian loop
    let _ = rltk::main_loop(context, gs);
}
