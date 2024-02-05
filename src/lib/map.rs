// use crate::*;
use crate::{rect::*, Player, Viewshed};
use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use std::cmp::{max, min};

// -----------------------Map section --------------------
// 公开的常量 地图的大小
pub const MAPWIDTH: usize = 80;
// 为屏幕底部的 UI 留出空间
pub const MAPHEIGHT: usize = 43;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

// 地图的类型，枚举
#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,  // “#”符号
    Floor, // “.” 符号
    DownStairs,
}
// 创建一个map struct ，存储与map 相关的数据
// 使用 宏 让Map 进行序列化
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub height: i32,
    pub width: i32,
    pub revealed_tiles: Vec<bool>, // 记录玩家看到过的地图
    pub visible_tiles: Vec<bool>,  // 将记住了但是看不到的内容变灰
    pub blocked: Vec<bool>,        // 哪些tile不可以走上去，防止玩家和怪物重叠
    // 深度 i32 is primitive type, and automatically handled by Serde,
    // So adding it here automatically adds it to our game save/load mechanism.
    pub depth: i32,
    // 存储地图上tile 的内容
    // 跳过 对 tile_conent 的序列化
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}
impl Map {
    // 如何将坐标映射魏地图数组的下标
    // （1，2） = 1 +2 * 80 = 161
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.height as usize) + x as usize
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

    // 水平的通道
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                // 在地图的范围内
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    // 垂直的通道
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                // 在地图的范围内
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    // 退出的有效,是否 是 房间的出口
    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        // 边界检查
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        // 不被阻挡的才可以exit
        !self.blocked[idx]
    }

    // 填充(populate)被阻挡的tile  wall was blocked
    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    // 清楚tile content 的 索引
    // 程序开辟内存是很慢的
    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    /// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't look awful.
    /// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
    /// This gives a handful of random rooms and corridors joining them together.
    // 可以为不同的level 创建地图
    pub fn new_map_rooms_and_corridors(new_depth: i32) -> Self {
        // now map is full for wall
        let mut map = Map {
            tiles: vec![TileType::Wall; MAPCOUNT],
            rooms: Vec::new(),
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles: vec![false; MAPCOUNT], // 最开始玩家没有看到任何一个tile
            visible_tiles: vec![false; MAPCOUNT],
            blocked: vec![false; MAPCOUNT],
            tile_content: vec![Vec::new(); MAPCOUNT],
            depth: new_depth,
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for i in 0..MAX_ROOMS {
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
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }
        // 放置楼梯， 一个房间的中心
        let stairs_position = map.rooms[map.rooms.len() - 1].center();
        let stairs_idx = map.xy_idx(stairs_position.0, stairs_position.1);
        map.tiles[stairs_idx] = TileType::DownStairs;

        // 返回所有的房间及地图，出参和入参之间没有联系，有，考虑生命周期
        map
    }
}

impl BaseMap for Map {
    // opaque 不透明的
    fn is_opaque(&self, idx: usize) -> bool {
        // 如果图块时墙，返回true 否则 返回 false
        self.tiles[idx as usize] == TileType::Wall
    }

    // 根据 tilt 索引 得到 这两个tile 之间的距离
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    // 获得可以的退出的tile
    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        // 将存在的tile 放进一个 vector
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width; // 这里的idx 是一维的 余数是y，除数是x
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions 四个方向 移动
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        // Diagonals 对角线方向移动
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - w) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + w) + 1, 1.45));
        }
        exits
    }
}

// RLTK 并不关心 我们的地图如何实现，只要你实现了他提供的trait ,RLTK 实现了对对应的trait 的逻辑
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

// 检查 tile 是否被露出 revealed 以及他是否是一面墙
fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x, y);
    map.tiles[idx] == TileType::Wall && map.revealed_tiles[idx]
}
// 绘制墙
fn wall_glyph(map: &Map, x: i32, y: i32) -> rltk::FontCharType {
    // map bounds, do not stepping outside of them, return a # symbol(ASCII 35)
    if x < 1 || x > map.width - 2 || y < 1 || y > map.height - 2 as i32 {
        return 35;
    }

    // create a 8-bit unsigned integer to act our bitmask, setting individual bits ana only need four of them, an 8-bit number is perfect
    let mut mask: u8 = 0;
    // check each of the 4 directions and add to the mask, a value of 3 means that we have neighbors to the north and south. 北 和 南

    // 北
    if is_revealed_and_wall(map, x, y - 1) {
        mask += 1;
    }
    // 南
    if is_revealed_and_wall(map, x, y + 1) {
        mask += 2;
    }
    if is_revealed_and_wall(map, x - 1, y) {
        mask += 4;
    }
    if is_revealed_and_wall(map, x + 1, y) {
        mask += 8;
    }
    // them we match on the resulting mask bit and return the appropriate line-drawing character from the codepage 437 character set
    match mask {
        0 => 9,    // Pillar because we can't see neighbors
        1 => 186,  // Wall only to the north
        2 => 186,  // Wall only to the south
        3 => 186,  // Wall to the north and south
        4 => 205,  // Wall only to the west
        5 => 188,  // Wall to the north and west
        6 => 187,  // Wall to the south and west
        7 => 185,  // Wall to the north, south and west
        8 => 205,  // Wall only to the east
        9 => 200,  // Wall to the north and east
        10 => 201, // Wall to the south and east
        11 => 204, // Wall to the north, south and east
        12 => 205, // Wall to the east and west
        13 => 202, // Wall to the east, west, and south
        14 => 203, // Wall to the east, west, and north
        15 => 206, // ╬ Wall on all sides
        _ => 35,   // We missed one?
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
                    // 通过画线字符绘制墙
                    glyph = wall_glyph(&*map, x, y);
                    fg = RGB::from_f32(0.0, 1.0, 0.0);
                } // 如果tile 被 revealed 但是 玩家不可见 visible， 被设置为白色
                TileType::DownStairs => {
                    glyph = rltk::to_cp437('>');
                    fg = RGB::from_f32(0., 1.0, 1.0);
                }
            }
            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale()
            }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);

            // Move the coordinates ，转到下一行
            x += 1;
            if x > MAPWIDTH as i32 - 1 {
                x = 0;
                y += 1;
            }
        }
    }
}
