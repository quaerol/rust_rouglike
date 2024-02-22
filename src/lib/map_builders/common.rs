use super::Map;
use crate::{Rect, TileType};
use std::{
    cmp::{max, min},
    collections::HashMap,
};

// 通用代码

// romm is a rect, map 中的rooms 存储的是应用到map 中的rooms
pub fn apply_room_to_map(map: &mut Map, room: &Rect) {
    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }
}

// 水平的通道
pub fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = map.xy_idx(x, y);
        if idx > 0 && idx < map.width as usize * map.height as usize {
            // 在地图的范围内
            map.tiles[idx as usize] = TileType::Floor;
        }
    }
}
// 垂直的通道
pub fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = map.xy_idx(x, y);
        if idx > 0 && idx < map.width as usize * map.height as usize {
            // 在地图的范围内
            map.tiles[idx as usize] = TileType::Floor;
        }
    }
}

// 将不能到达tile 这是为墙，返回出口tile 的索引
pub fn remove_unreachable_areas_returning_most_distant(map: &mut Map, start_idx: usize) -> usize {
    map.populate_blocked();
    // find all tiles we can reach from the starting point
    // map_starts 的向量，并给它一个值：玩家开始的图块索引。 Dijkstra 地图可以有多个起点（距离 0），因此即使只有一个选择，它也必须是一个向量
    let map_starts: Vec<usize> = vec![start_idx];
    // 要求 RLTK 为我们制作一张 Dijkstra 地图。它的尺寸与主地图相匹配，使用起点，具有对地图本身的读取权限，并且我们将在 200 步时停止计数
    let dijkstra_map = rltk::DijkstraMap::new(
        map.width as usize,
        map.height as usize,
        &map_starts,
        map,
        300.0,
    );
    // exit_tile tuple 设置为 0 和 0.0 。第一个零是出口的瓦片索引，第二个零是到出口的距离。
    let mut exit_tile = (0, 0.0f32);
    // 单元格索引添加为元组中的第一个参数。然后我们解构以获得图块和索引
    for (i, tile) in map.tiles.iter_mut().enumerate() {
        if *tile == TileType::Floor {
            // 从 Dijkstra 地图中获取该 tile 到起点的距离
            let distance_to_start = dijkstra_map.map[i];
            // we can not get to this tile - so we will make it a wall
            if distance_to_start == std::f32::MAX {
                *tile = TileType::Wall;
            } else {
                // if it is further away than our current exit candidate, move the exit
                // 如果距离大于 exit_tile 元组中的距离，我们将存储新距离和新图块索引。
                if distance_to_start > exit_tile.1 {
                    exit_tile.0 = i;
                    exit_tile.1 = distance_to_start;
                }
            }
        }
    }

    exit_tile.0
}

// voronoi 泰森多边形
/// Generates a Voronoi/cellular noise map of a region, and divides it into spawn regions.
#[allow(clippy::map_entry)]
pub fn generate_voronoi_spawn_regions(
    map: &Map,
    rng: &mut rltk::RandomNumberGenerator,
) -> HashMap<i32, Vec<usize>> {
    let mut noise_areas: HashMap<i32, Vec<usize>> = HashMap::new();
    let mut noise = rltk::FastNoise::seeded(rng.roll_dice(1, 65536) as u64);
    noise.set_noise_type(rltk::NoiseType::Cellular);
    noise.set_frequency(0.08);
    noise.set_cellular_distance_function(rltk::CellularDistanceFunction::Manhattan);

    for y in 1..map.height - 1 {
        for x in 1..map.width - 1 {
            let idx = map.xy_idx(x, y);
            if map.tiles[idx] == TileType::Floor {
                let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.0;
                let cell_value = cell_value_f as i32;

                if noise_areas.contains_key(&cell_value) {
                    noise_areas.get_mut(&cell_value).unwrap().push(idx);
                } else {
                    noise_areas.insert(cell_value, vec![idx]);
                }
            }
        }
    }

    noise_areas
}
