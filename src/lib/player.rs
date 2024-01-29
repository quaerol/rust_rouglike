use crate::*;
pub use map::*;
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use specs_derive::Component;
use std::cmp::{max, min};

// player move
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    // 获得战斗状态组件的存储器
    let combat_stats = ecs.read_storage::<CombatStats>();
    // 攻击意图组件的存储器
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    // 得到所有的实体
    let entities = ecs.entities();

    // let map = ecs.fetch::<Vec<TileType>>();
    let map = ecs.fetch::<Map>();

    for (entity, _player, pos, viewshed) in
        (&entities, &mut players, &mut positions, &mut viewsheds).join()
    {
        if pos.x + delta_x < 1
            || pos.x + delta_x > map.width - 1
            || pos.y + delta_y < 1
            || pos.y + delta_y > map.height - 1
        {
            return;
        }
        // 目标位置 索引
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        // 潜在的目标
        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                // 给 entity 插入 WantsToMelee 组件
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed");
                return;
            }
        }
        // 现在不会越过Wall 也不会 踩过（walking over） 怪物
        if !map.blocked[destination_idx] {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            // 玩家移动，看到的东西已经改变，viewshed 的 dirty 标志改变
            viewshed.dirty = true;

            // 在玩家移动的时候更新 玩家位置的资源
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}
fn get_item(ecs: &mut World) {
    // obtains a bunch of references/accessors (访问器 和引用器) from the ECS, and iterates all items with a position
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    let mut target_item: Option<Entity> = None;
    // 如果玩家的位置和物品的位置重合
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        // 打印日志
        None => gamelog
            .entries
            .push("There is nothing here to pick up.".to_string()),
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup
                .insert(
                    *player_entity,
                    WantsToPickupItem {
                        collected_by: *player_entity,
                        item,
                    },
                )
                .expect("Unable to insert want to pickup");
        }
    }
}
pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    if let Some(key) = ctx.key {
        match key {
            // 四方移动
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, &mut gs.ecs)
            }

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, &mut gs.ecs)
            }

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, &mut gs.ecs)
            }

            // Diagonals 对角线移动
            VirtualKeyCode::Numpad9 | VirtualKeyCode::Y => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 | VirtualKeyCode::U => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

            // G 键 拾取 物品
            VirtualKeyCode::G => get_item(&mut gs.ecs),
            // i 键显示库存
            VirtualKeyCode::I => return RunState::ShowInventory,
            // D 显示丢弃菜单
            VirtualKeyCode::D => return RunState::ShowDropItem,

            // Save and Quit
            VirtualKeyCode::Escape => return RunState::SaveGame,

            // Level changes
            VirtualKeyCode::Period => {
                if try_next_level(&mut gs.ecs) {
                    // 游戏的状态
                    return RunState::NextLevel;
                }
            }
            _ => {
                return RunState::AwaitingInput;
            }
        }
    }

    RunState::PreRun
}

pub fn try_next_level(ecs:&mut World) -> bool{
    let player_pos = ecs.fetch::<point>();
    let map = ecs.fetch::<Map>();
}