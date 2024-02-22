use super::{
    apply_room_to_map, spawner, Map, MapBuilder, Position, Rect, TileType, SHOW_MAPGEN_VISUALIZER,
};
use rltk::RandomNumberGenerator;
use specs::prelude::*;

pub struct BspDungeonBuilder {
    map: Map,
    starting_position: Position,
    depth: i32,
    rooms: Vec<Rect>,
    history: Vec<Map>,
    rects: Vec<Rect>,
}

impl BspDungeonBuilder {
    pub fn new(new_depth: i32) -> BspDungeonBuilder {
        BspDungeonBuilder {
            map: Map::new(new_depth),
            starting_position: Position { x: 0, y: 0 },
            depth: new_depth,
            rooms: Vec::new(),
            history: Vec::new(),
            rects: Vec::new(),
        }
    }

    // 地图的构建函数
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        self.rects.clear();
        self.rects
            .push(Rect::new(2, 2, self.map.width - 5, self.map.height - 5)); // Start with a single map-sized rectangle
                                                                             // 创建第一个房间，这个房间实际上是整个地图
        let first_room = self.rects[0];
        // 对第一个地图进行一些修剪，在地图的两侧进行一些填充
        // 方法 add_subrects，将一个矩形分为四个象限，然后将每个象限太添加到矩形列表
        self.add_subrects(first_room); // Divide the first room

        // 设置一个房间计数器，避免无限循环
        // Up to 240 times, we get a random rectangle and divide it. If its possible to squeeze a
        // room in there, we place it and add it to the rooms list.
        let mut n_rooms = 0;
        while n_rooms < 240 {
            // 得到一个随机的矩形
            let rect = self.get_random_rect(&mut rng);
            // candidate 候选
            // 利用一个矩形作为边界，得到一个随机大小的房间
            let candidate = self.get_random_sub_rect(rect, &mut rng);
            // 是否可以将整个候选房间加到地图上
            if self.is_possible(candidate) {
                apply_room_to_map(&mut self.map, &candidate);
                self.rooms.push(candidate);
                // 调用 add_subrects 来细分我们刚刚使用的矩形（不是候选！）
                self.add_subrects(rect);
                // 拍摄地图的快照
                self.take_snapshot();
            }

            n_rooms += 1
        }

        // now we sort the rooms, 根据左坐标排序
        self.rooms.sort_by(|a, b| a.x1.cmp(&b.x1));

        // now we want corridors
        for i in 0..self.rooms.len() - 1 {
            let room = self.rooms[i];
            let next_room = self.rooms[i + 1];

            // 选择起始房间中一个随机位置作为走廊的开始，和 下一个房间中的一个随机位置作为走廊的结束
            let start_x = room.x1 + (rng.roll_dice(1, i32::abs(room.x1 - room.x2)) - 1);
            let start_y = room.y1 + (rng.roll_dice(1, i32::abs(room.y1 - room.y2)) - 1);
            let end_x =
                next_room.x1 + (rng.roll_dice(1, i32::abs(next_room.x1 - next_room.x2)) - 1);
            let end_y =
                next_room.y1 + (rng.roll_dice(1, i32::abs(next_room.y1 - next_room.y2)) - 1);
            // 以开始坐标和结束坐标来绘制走廊
            self.draw_corridor(start_x, start_y, end_x, end_y);
            // 绘制一个走廊后，拍摄快照，作为可视化加载的数据
            self.take_snapshot();
        }

        // do not forget the stairs
        // 最后一个房间的中心点走位next level 的 楼梯
        let stairs = self.rooms[self.rooms.len() - 1].center();
        let stairs_idx = self.map.xy_idx(stairs.0, stairs.1);
        self.map.tiles[stairs_idx] = TileType::DownStairs;
        self.take_snapshot();

        // set player start
        let start = self.rooms[0].center();
        self.starting_position = Position {
            x: start.0,
            y: start.1,
        };
    }
    // 构建地图
    // 将一个矩形按照四个象限分为四个矩形, 并放入矩形列表中,BSP 算法的核心
    fn add_subrects(&mut self, rect: Rect) {
        let width = i32::abs(rect.x1 - rect.x2);
        let height = i32::abs(rect.y1 - rect.y2);
        let half_width = i32::max(width / 2, 1);
        let half_height = i32::max(height / 2, 1);

        self.rects
            .push(Rect::new(rect.x1, rect.y1, half_width, half_height));
        self.rects.push(Rect::new(
            rect.x1,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
    }

    // 从矩形列表中得到一个随机的矩形，
    fn get_random_rect(&mut self, rng: &mut RandomNumberGenerator) -> Rect {
        if self.rects.len() == 1 {
            return self.rects[0];
        }
        let idx = (rng.roll_dice(1, self.rects.len() as i32) - 1) as usize;
        self.rects[idx]
    }
    // 从一个矩形中随机分割出一个小的矩形
    fn get_random_sub_rect(&self, rect: Rect, rng: &mut RandomNumberGenerator) -> Rect {
        // 复制一个rect
        let mut result = rect;
        let rect_width = i32::abs(rect.x1 - rect.x2);
        let rect_height = i32::abs(rect.y1 - rect.y2);

        // 随机长宽的取值
        // no less than 3 tiles in size and no more than 10 on each dimension.
        let w = i32::max(3, rng.roll_dice(1, i32::min(rect_width, 10)) - 1) + 1;
        let h = i32::max(3, rng.roll_dice(1, i32::min(rect_height, 10)) - 1) + 1;

        // 两个点确定一个矩形
        result.x1 += rng.roll_dice(1, 6) - 1;
        result.y1 += rng.roll_dice(1, 6) - 1;
        result.x2 = result.x1 + w;
        result.y2 = result.y1 + h;

        result
    }

    // 这个矩形是否符合要求，更新 build
    fn is_possible(&self, rect: Rect) -> bool {
        let mut expanded = rect;
        // 将矩形的长宽+2 作为 扩展矩形
        expanded.x1 -= 2;
        expanded.x2 += 2;
        expanded.y1 -= 2;
        expanded.y2 += 2;

        let mut can_build = true;

        // 遍历扩展矩形
        for y in expanded.y1..=expanded.y2 {
            for x in expanded.x1..=expanded.x2 {
                // 如果扩展矩形的长宽超过了地图的边界，则该矩形，不能build
                if x > self.map.width - 2 {
                    can_build = false;
                }
                if y > self.map.height - 2 {
                    can_build = false;
                }
                if x < 1 {
                    can_build = false;
                }
                if y < 1 {
                    can_build = false;
                }
                // 地图的类型是墙不能构建
                if can_build {
                    let idx = self.map.xy_idx(x, y);
                    if self.map.tiles[idx] != TileType::Wall {
                        can_build = false;
                    }
                }
            }
        }
        can_build
    }

    // 绘制走廊,
    fn draw_corridor(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let mut x = x1;
        let mut y = y1;

        // 开始的坐标和结束的坐标 x y 任意一个相等代表走廊到头
        while x != x2 || y != y2 {
            if x < x2 {
                x += 1;
            } else if x > x2 {
                x -= 1;
            } else if y < y2 {
                y += 1;
            } else if y > y2 {
                y -= 1;
            }

            let idx = self.map.xy_idx(x, y);
            self.map.tiles[idx] = TileType::Floor;
        }
    }
}
impl MapBuilder for BspDungeonBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }
    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }
    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }
    fn build_map(&mut self) {
        self.build();
    }
    fn spawn_entities(&mut self, ecs: &mut World) {
        for room in self.rooms.iter().skip(1) {
            spawner::spawn_room(ecs, room, self.depth);
        }
    }
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.revealed_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }
}
