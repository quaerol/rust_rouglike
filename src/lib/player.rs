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

    // let map = ecs.fetch::<Vec<TileType>>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let des_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[des_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            // 玩家移动，看到的东西已经改变，viewshed 的 dirty 标志改变
            viewshed.dirty = true;

            // 在玩家移动的时候更新 玩家位置的资源 将玩家位置写入Point 资源
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

// 玩家拾取物品
fn get_item(ecs:&mut World){
    // 拾取物品需要哪些实体，需要哪些组件，玩家 玩家的位置 item position 打印日志
    // 从ecs 中获得 a bunch of references/accessors
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    // ReadStorage 表示只读的组件存储
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    let mut target_item:Option<Entity> = None;
    // join() 方法则用于将多个组件存储合并为一个迭代器，以便同时迭代它们。
    for (item_entity,_item,position) in (&entities,&item,&positions).join(){
        // 如果玩家和item 的位置相同，拾取该物品
        if position.x == player_pos.x && position.y == player_pos.y{
            target_item = Some(item_entity);
        } 
    }
    match target_item{
        None => gamelog.entries.push("There is nothing here to pick up.".to_string()),
        Some(item) =>{
            // 从ecs （是一个World 类型）中 创建一个 WantsToPickupItem 的组件存储
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            // 将拾取的物品插入 组件存储
            pickup.insert(*player_entity,WantsToPickupItem{
                collected_by: *player_entity,
                item
            }).expect("Unable to insert want to pickup");

        }
    }
}
// 按键处理
pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    if let Some(key) = ctx.key {
        match key {
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
            // G 键 拾取 物品
            VirtualKeyCode::G => {
                get_item(&mut gs.ecs)
            }
            // I 键位 列出库存
            VirtualKeyCode::I => return RunState::ShowInventory,
            // 显示丢弃的物品
            VirtualKeyCode::D => return RunState::ShowDropItem,
            _ => {
                return RunState::Paused;
            }
        }
    }

    RunState::Running
}
