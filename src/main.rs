use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;

// 创建组件
#[derive(Component0)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct LeftMover {}

#[derive(Component)]
struct Player {}
// *****system*********************

// LeftWalker system
struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    // trait 也是需要的数据的，trait 的数据是哪来的的，来自她正在实现的struct
    type System = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>); // 移动需要修改位置的数据
    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

// player move
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Palyer>();

    let map = ecs.fetch::<vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let des_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[des_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}
// -----------------------Map section --------------------
// 地图的类型，枚举
enum TileType {
    Wall,  // “#”符号
    Floor, // “.” 符号
}
pub fn xy_idx(x: i32, y: i32) -> usize {
    todo!();
}

fn new_map() -> Vec<TileType> {
    todo!();
    // 包围的墙壁和地图内随机位置的墙
}
fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    todo!();
    // ctx.set()
}
// ------------------------World state section------------------------
struct State {
    ecs: World,
}
impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs); // LeftWalker 的run  需要修改 State 中的数据
        self.ecs.maintain();
    }
}
impl GameState for State {
    // 每一帧运行
    fn tick(&mut self, ctx: &mut Rltk) {
        // run system
        self.run_systems();

        // 清楚屏幕 clearn
        ctx.cls();

        let map = self.ecs.fetch::<Vec<TileType>>(); // 从world 中得到地图数据
        draw_map(&map, ctx);

        // get entity with component
        let positions = self.ecs.read_storage::<Position>(); // 这是什么用法
        let renderables = self.ecs.read_storage::<Renderable>(); // 这是什么用法
                                                                 // draw entities
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() {
    // use builder
    use rltk::RltkBuilder;

    // initialise context
    let context = RltkBuilder::simple80x50();

    // create world,because entity will add in world,so world is mutable
    let mut gs = State { ecs: World::new() };

    // -------------register component
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftWalker>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    // --------------add source in world  ,shared data the whole ecs can use
    gs.ecs.insert(new_map());

    // ------------------create entity
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437("x"),
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
                glyph: rltk::to_cp437("x"),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {}) // LeftMover 标记需要移动的
            .build();
    }

    // game mian loop
    rltk::main_loop(context, gs);
}
