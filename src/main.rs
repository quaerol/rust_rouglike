use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;

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
