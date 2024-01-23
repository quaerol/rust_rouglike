use rltk::{GameState, Rltk};
use specs::Join;
use specs::RunNow;
use specs::World;
use specs::WorldExt;

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

// ------------------------World state section------------------------

// turn-base game,回合制游戏，game state 游戏循环的状态
//Copy 将其标记为“复制”类型 - 它可以安全地复制到内存中（意味着它没有会被搞乱的指针）。 Clone 悄悄地为其添加了 .clone() 功能，允许您以这种方式进行内存复制。
#[derive(PartialEq, Copy, Clone, Debug)] // derive 宏，自动为下面的结构实现这些trait , partialeq runstate 之间可以比较
pub enum RunState {
    AwaitingInput,
    Prerun,
    PlayerTurn,
    MonsterTurn,
    // 展示库存的状态
    ShowInventory,
    // 显示可以丢弃物品的菜单
    ShowDropItem,
    // 显示攻击目标
    ShowTargeting { range : i32, item : Entity}
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

        // 物品收集的系统
        let mut pickup = ItemCollectionSystem{};
        pickup.run_now(&self.ecs);

        // 药水使用系统
        let mut potions = PotionUseSystem{};
        potions.run_now(&self.ecs);

        // 物品丢弃系统
        let mut drop_items = ItemDropSystem{};
        drop_items.run_now(&self.ecs);

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
        let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
        data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order) );
        for (pos, render) in data.iter(){
            let idx = map.xy_idx(pos.x, pos.y);
            // 检查怪物占用的 tile 是否可见，
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }

        // 根据游戏循环不同的状态， 执行不同的操作
        // 显示库存
        RunState::ShowInventory => {
            // 得到 使用的物品
            let result = gui::show_inventory(self,ctx);
            match result.0 {
                gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                gui::ItemMenuResult::NoResponse => {}
                gui::ItemMenuResult::Selected => {
                    // 得到选中的物体
                    let item_entity = result.1.unwarp();
                    // 为选中的物体 添加 WantsToDrinkPotion 组件，标记可以被饮用
                    let is_ranged= self.ecs.read_storage::<Ranged>();
                    let is_item_ranged = is_ranged.get(item_entity);

                    if let Some(is_item_ranged) = is_item_ranged {
                        newrunstate = RunState::ShowTargeting{ range: is_item_ranged.range, item: item_entity };
            
                    }else{
                        let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                        let names = self.ecs.read_storage::<Name>();
                        let mut gamelog = self.ecs.fetch_mut::<gamelog::GameLog>();
                        gamelog.entries.push(format!("You try to use {}", names.get(item_entity).unwrap().name));
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToUseItem{ item: item_entity, target: None }).expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
        }

        // 显示丢弃的物品
        RunState::ShowDropItem =>{
            // 丢弃 物品的菜单
            let result = gui::drop_item_menu(self,ctx);
            // 匹配物品菜单的状态
            match result.0 {
                gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                gui::ItemMenuResult::NoResponse => {}
                gui::ItemMenuResult::Selected => {
                    // 得到 待 丢弃的物品
                    let item_entity = result.1.unwrap();
                    // 想要 丢弃 物品 的意图
                    let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                    intent.insert(*self.ecs.fetch::<Entity>(), WantsToDropItem{ item: item_entity }).expect("Unable to insert intent");
                    // 改变运行状态
                    newrunstate = RunState::PlayerTurn;
                }
            }
        }

        // 
    }
}
