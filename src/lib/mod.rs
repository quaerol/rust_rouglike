pub mod rect;
pub use rect::*;
pub mod components;
pub use components::*;
pub mod map;
pub use map::*;
pub mod player;
pub use player::*;
pub mod visibility_system;
use rltk::{GameState, Rltk};
use specs::Join;
use specs::RunNow;
use specs::World;
use specs::WorldExt;
pub use visibility_system::*;

// ------------------------World state section------------------------
pub struct State {
    pub ecs: World,
}
impl State {
    // 系统的调度程序
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs); // 这里运行实际的系统
                                // let mut lw = LeftWalker {};
                                // lw.run_now(&self.ecs); // LeftWalker 的run  需要修改 State 中的数据
        self.ecs.maintain();
    }
}
impl GameState for State {
    // 每一帧运行
    fn tick(&mut self, ctx: &mut Rltk) {
        // 清楚屏幕 clearn
        ctx.cls();

        player_input(self, ctx);
        // run system
        self.run_systems();

        // 从world 中得到地图数据
        // let map = self.ecs.fetch::<Map>();
        draw_map(&self.ecs, ctx);

        // get entity with component
        let positions = self.ecs.read_storage::<Position>(); // 这是什么用法
        let renderables = self.ecs.read_storage::<Renderable>(); // 这是什么用法
                                                                 // draw entities
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
