use rltk::{GameState, Rltk};
use serde::*;
use specs::Entity;
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

pub mod map_indexing_system;
pub use map_indexing_system::*;

mod melee_combat_system;
use melee_combat_system::*;

mod damage_system;
use damage_system::*;

pub mod gui;
pub use gui::*;

pub mod gamelog;
pub use gamelog::*;

pub mod inventory_system;
pub use inventory_system::*;

pub mod spawner;
pub use spawner::*;

pub mod saveload_system;
pub use saveload_system::*;

// 随机常见 生成表
pub mod random_table;
pub use random_table::*;

// ------------------------World state section------------------------
// turn-base game,回合制游戏，game state
//Copy 将其标记为“复制”类型 - 它可以安全地复制到内存中（意味着它没有会被搞乱的指针）。 Clone 悄悄地为其添加了 .clone() 功能，允许您以这种方式进行内存复制。
// derive 宏，自动为下面的结构实现这些trait , partialeq runstate 之间可以比较
// 游戏的不同状态
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    // 显示库存
    ShowInventory,
    ShowDropItem,
    // 显示攻击目标
    ShowTargeting {
        range: i32,
        item: Entity,
    },
    // 处于菜单，存储当前的选项, gui 绘制菜单
    MainMenu {
        menu_selection: gui::MainMenuSelection,
    },
    // 保存游戏的状态
    SaveGame,
    NextLevel,
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

        // 填充 被阻挡tile 的系统
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);

        // 战斗系统
        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);

        // 攻击伤害系统
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);

        // 物品拾取系统
        let mut pickup = ItemCollectionSystem {};
        pickup.run_now(&self.ecs);

        // 物品使用系统
        let mut potions = ItemUseSystem {};
        potions.run_now(&self.ecs);

        // 物品 丢弃系统
        let mut drop_items = ItemDropSystem {};
        drop_items.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl State {
    // 去到下一层，房间内 生成点 和生成的内容变化
    fn goto_next_level(&mut self) {
        // Build a new map and place the player
        let worldmap;
        let current_depth;
        {
            let mut worldmap_resource = self.ecs.write_resource::<Map>();
            current_depth = worldmap_resource.depth;
            *worldmap_resource = Map::new_map_rooms_and_corridors(current_depth + 1);
            worldmap = worldmap_resource.clone();
        }

        // Spawn bad guys
        for room in worldmap.rooms.iter().skip(1) {
            spawner::spawn_room(&mut self.ecs, room, current_depth + 1);
        }
    }
}

impl GameState for State {
    // 每一帧运行
    fn tick(&mut self, ctx: &mut Rltk) {
        // ---------------------- game state ---------------------------------------------------
        // 基于回合制的滴答循环
        // 在运行状态下运行系统
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        // 清楚屏幕 clearn
        ctx.cls();

        // 显示 菜单的时候不会 同时渲染GUI 和 地图
        match newrunstate {
            // handle the mainmenu state in our large match, 处理 处于菜单的状态
            RunState::MainMenu { .. } => {}
            _ => {
                // --------------------render ---------------------------------------------------
                // 从world 中得到地图数据
                // let map = self.ecs.fetch::<Map>();
                draw_map(&self.ecs, ctx);
                {
                    // get entity with component 通过组件找到实体 ，这里 是玩家和怪物
                    let positions = self.ecs.read_storage::<Position>();
                    let renderables = self.ecs.read_storage::<Renderable>();

                    // 拿到地图
                    let map = self.ecs.fetch::<Map>();

                    // draw entities 根据渲染顺序 绘制 player monster item 等 实体
                    let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
                    // 根据 render_order 进行 排序
                    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));

                    for (pos, render) in data.iter() {
                        let idx = map.xy_idx(pos.x, pos.y);
                        // 怪物占用的 tile 是否可见，
                        if map.visible_tiles[idx] {
                            // 渲染绘制 entity
                            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                        }
                    }

                    // 绘制UI
                    gui::draw_ui(&self.ecs, ctx);
                }
            }
        }

        match newrunstate {
            RunState::PreRun => {
                // run system dispatch system
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                // 怪物只会在玩家移动是考虑做什么
                self.runstate = player_input(self, ctx);
            }
            // 玩家 回合 和 怪物 回合
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::ShowInventory => {
                // 如果在库存状态下按下 Cancel 退出 ShowInventory 状态
                let result = gui::show_inventory(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        // 日志打印选中的物品 name
                        let item_entity = result.1.unwrap();
                        // 得到 Ranged 的组件存储器
                        let is_ranged = self.ecs.read_storage::<Ranged>();
                        // 得到选中物体中的数据
                        let is_item_ranged = is_ranged.get(item_entity);
                        let names = self.ecs.read_storage::<Name>();
                        let mut gamelog = self.ecs.fetch_mut::<gamelog::GameLog>();
                        gamelog.entries.push(format!(
                            "You try to use {}, but it isn't written yet",
                            names.get(item_entity).unwrap().name
                        ));
                        if let Some(is_item_ranged) = is_item_ranged {
                            // 改变状态，并给初始化 ShowTargeting 状态
                            newrunstate = RunState::ShowTargeting {
                                range: is_item_ranged.range,
                                item: item_entity,
                            };
                        } else {
                            let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                            // create a WantsToDrinkPotion intent:
                            intent
                                .insert(
                                    *self.ecs.fetch::<Entity>(),
                                    WantsToUseItem {
                                        item: item_entity,
                                        target: None,
                                    },
                                )
                                .expect("Unable to insert intent");
                            newrunstate = RunState::PlayerTurn;
                        }
                    }
                }
            }

            // 在 ShowDropItem 游戏状态下 doing something
            RunState::ShowDropItem => {
                // 1 显示 drop_item_menu
                let result = gui::drop_item_menu(self, ctx);
                match result.0 {
                    // 2 得到菜单后 菜单的状态 的 操作
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        // 3 丢弃选中的物品, 得到选中的物品
                        let item_entity = result.1.unwrap();
                        // 得到 WantsToDropItem 的 存储组件
                        let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                        // 在这个存储组件插入 这个 选中的实体
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToDropItem { item: item_entity },
                            )
                            .expect("Unable to insert intent");
                        // 改变 游戏 运行状态
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }

            // 在 显示攻击选择菜单
            RunState::ShowTargeting { range, item } => {
                let result = gui::ranged_target(self, ctx, range);
                // 根据选项菜单的结果进行匹配
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        // 将攻击目标放入 WantsToUseItem 意图组件存储器中
                        let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToUseItem {
                                    item,
                                    target: result.1,
                                },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::MainMenu { .. } => {
                // 得到菜单及其选项
                let result = gui::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection { selected } => {
                        newrunstate = RunState::MainMenu {
                            menu_selection: selected,
                        }
                    }
                    gui::MainMenuResult::Selected { selected } => match selected {
                        gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                        gui::MainMenuSelection::LoadGame => {
                            // 从json  文件中加载地图，反序列化
                            saveload_system::load_game(&mut self.ecs);
                            newrunstate = RunState::AwaitingInput;
                            // 重新加载后，删除游戏存档文件
                            saveload_system::delete_save();
                        }
                        gui::MainMenuSelection::Quit => {
                            ::std::process::exit(0);
                        }
                    },
                }
            }
            // 处理 这个 状态 下的逻辑
            RunState::SaveGame => {
                saveload_system::save_game(&mut self.ecs);
                newrunstate = RunState::MainMenu {
                    menu_selection: gui::MainMenuSelection::Quit,
                };
            }
            RunState::NextLevel => {
                todo!()
            }
        }

        {
            // 存储这一游戏循环结束 游戏的状态
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        // 删除死亡实体
        damage_system::delete_the_dead(&mut self.ecs);
    }
}
