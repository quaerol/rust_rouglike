pub mod rect;
pub use rect::*;
pub mod components;
pub use components::*;
pub mod map;
pub use map::*;
pub mod player;
pub use player::*;
pub mod visibility_system;
pub use visibility_system::*;
pub mod monster_ai_system;
pub use monster_ai_system::*;
use rltk::{GameState, Rltk};
use specs::Join;
use specs::RunNow;
use specs::World;
use specs::WorldExt;

// ------------------------World state section------------------------
// turn-base game,回合制游戏，game state
//Copy 将其标记为“复制”类型 - 它可以安全地复制到内存中（意味着它没有会被搞乱的指针）。 Clone 悄悄地为其添加了 .clone() 功能，允许您以这种方式进行内存复制。
#[derive(PartialEq, Copy, Clone, Debug)] // derive 宏，自动为下面的结构实现这些trait , partialeq runstate 之间可以比较
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}
impl State {
    // 系统的调度程序
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs); // 这里运行实际的系统

        // 运行怪物的AI系统
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
impl GameState for State {
    // 每一帧运行
    fn tick(&mut self, ctx: &mut Rltk) {
        // 清楚屏幕 clearn
        ctx.cls();

        // 基于回合制的滴答循环
        // 在运行状态下运行系统
        if self.runstate == RunState::Running {
            // run system
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            // 怪物只会在玩家移动是考虑做什么
            self.runstate = player_input(self, ctx);
        }
        // 从world 中得到地图数据
        // let map = self.ecs.fetch::<Map>();
        draw_map(&self.ecs, ctx);

        // 渲染循环
        // get entity with component 通过组件找到实体 ，这里 是玩家和怪物
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        // 拿到地图
        let map = self.ecs.fetch::<Map>();

        // draw entities
        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            // 检查怪物占用的 tile 是否可见，
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}
