// use crate::*;
use crate::{rect::*, Player, Viewshed};
use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;
use std::cmp::{max, min};
// -----------------------Map section --------------------
// 公开的常量 地图的大小
pub const MAPWIDTH : usize = 80;
pub const MAPHEIGHT : usize = 43;
pub const MAPCOUNT : usize = MAPHEIGHT * MAPWIDTH;

// 地图的类型，枚举
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,  // “#”符号
    Floor, // “.” 符号
}
// 创建一个map struct ，存储与map 相关的数据
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub height: i32,
    pub width: i32,
    pub revealed_tiles: Vec<bool>, // 记录玩家看到过的地图
    pub visible_tiles: Vec<bool>,  // 将记住了但是看不到的内容变灰
}
impl Map {
    // 如何将坐标映射魏地图数组的下标
    // （1，2） = 1 +2 * 80 = 161
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.height as usize) + x as usize
    }
    // documentation tags comment
    /// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't look awful.

    pub fn new_map_rooms_and_corridors() -> Self {
        // now map is full for wall
        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            revealed_tiles: vec![false; 80 * 50], // 最开始玩家没有看到任何一个tile
            visible_tiles: vec![false; 80 * 50],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);

            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;

            let new_room = Rect::new(x, y, w, h);

            let mut ok = true;

            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    // 如果两个房间重叠
                    ok = false;
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                // 将房间用走廊连在一起
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        // 走廊的宽度不同
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_x, new_y, prev_x);
                    } else {
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map // 返回所有的房间及地图，出参和入参之间没有联系，有，考虑生命周期
    }
    // 水平的通道
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx <= 80 * 50 {
                // 在地图的范围内
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    // 垂直的通道
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx <= 80 * 50 {
                // 在地图的范围内
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    // romm is a rect, map 中的rooms 存储的是应用到map 中的rooms
    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..room.y2 {
            for x in room.x1 + 1..room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
}

// RLTK 并不关心 我们的地图如何实现，只要你实现了他提供的trait ,RLTK 实现了对对应的trait 的逻辑
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    // opaque 不透明的
    fn is_opaque(&self, idx: usize) -> bool {
        // 如果图块时墙，返回true 否则 返回 false
        self.tiles[idx as usize] == TileType::Wall
    }
}
// retrieve the the map and the player's viewshed ,it only draw tiles present in the viewshed
pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    // write_storage 从ecs 中拿到注册的组件，设置为可写的存储
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            // 是否被玩家看过
            let glyph;
            let mut fg;
            // 如果这个tile 被玩家看到过，才render
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.0, 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_f32(0.0, 1.0, 0.0);
                } // 如果tile 被 revealed 但是 玩家不可见 visible， 被设置为白色
            }
            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale()
            }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);

            // Move the coordinates ，转到下一行
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }
    }
}
