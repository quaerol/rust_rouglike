use crate::*;
pub use map::*;
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use specs_derive::Component;
use std::cmp::{max, min};

#[derive(Component)]
pub struct LeftMover {}

// *****system*********************
// LeftWalker system
#[derive(Component)]
pub struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    // trait 也是需要的数据的，trait 的数据是哪来的的，来自她正在实现的struct
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>); // 移动需要修改位置的数据
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
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
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
            _ => {}
        }
    }
}
