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

    // -------------register component
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    // gs.ecs.register::<LeftWalker>();
    // gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    gs.ecs.register::<Viewshed>(); // 将组件注册到系统中
    gs.ecs.register::<BlocksTile>();

    // --------------add source in world  ,shared data the whole ecs can use
    let map = Map::new_map_rooms_and_corridors();
    // 玩家的位置在第一个房间的中心位置
    let (player_x, player_y) = map.rooms[0].center();

    // 将玩家的位置作为 资源 插入 ecs 中 Point 是表示玩家位置的资源
    gs.ecs.insert(Point::new(player_x, player_y));

    // ------------------create entity
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
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
        .with(Name {
            name: "Player".to_string(),
        })
        .build();

    // ----------------------怪物生成器代码------------------------
    // 在每个房间的中间添加一个怪物
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        // 怪物的种类 1 是 哥布林 其余的是 半兽人
        match roll {
            1 => {
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = rltk::to_cp437('o');
                name = "Orc".to_string();
            }
        }
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .with(BlocksTile {})
            .build();
    }

    // ----------------------------------------------------------------
    // 将map 插到world 中
    gs.ecs.insert(map);

    // game mian loop
    rltk::main_loop(context, gs)
}
