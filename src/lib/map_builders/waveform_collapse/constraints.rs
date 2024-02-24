use super::{Map, TileType};
use std::collections::HashSet;

// 这段代码应该会为您提供图像源文件中的每个 7x7 图块 
pub fn build_patterns(map : &Map, chunk_size: i32, include_flipping: bool, dedupe: bool) -> Vec<Vec<TileType>> {
    let chunks_x = map.width / chunk_size;
    let chunks_y = map.height / chunk_size;
    let mut patterns = Vec::new();

    for cy in 0..chunks_y {
        for cx in 0..chunks_x {
            // Normal orientation 正常的方向
            // 保存图案
            let mut pattern = Vec<TileType> = Vec::new();
            // 计算 start_x 、 end_x 、 start_y 和 end_y 来保存该块chunk在原始地图上的四个角坐标。
            let start_x = cx * chunk_size;
            let end_x = (cx +1) * chunk_size;
            let start_y = cx * chunk_size;
            let end_y = (cx +1) * chunk_size;

            // 按 y / x 顺序迭代图案chunk（以匹配我们的地图格式），
            for y in start_y..end_y {
                for x in start_x..end_x {
                    // 读取块内chunk 对应的地图中图块的 TileType ，然后添加它到图案chunk中。
                    let idx = map.xy_idx(x, y);
                    pattern.push(map.tiles[idx]);
                }
            }
            patterns.push(pattern);

            // 如果翻转图案
            if include_flipping {
                // filp horizontal 水平翻转
                for y in start_y..end_y{
                    for x in start_x..end_x{
                        // 从右到左进行，将地图的tile映射到图案中
                        let idx = map.xy_idx(end_x - (x+1), y);
                        pattern.push(map.tiles[idx]);
                    }
                }
                patterns.push(pattern);

                // Flip vertical 垂直翻转 x 不变 y 变化
                pattern = Vec::new();
                for y in start_y..end_y{
                    for x in start_x..end_x[
                        let idx = map.xy_idx(x,end_y - (y+1));
                        pattern.push(map.tiles[idx]);
                    ]
                }
                patterns.push(pattern);

                // flip both
                for y in start_y..end_y{
                    for x in start_x..end_x[
                        let idx = map.xy_idx(end_x - (x+1), end_y - (y+1));
                        pattern.push(map.tiles[idx]);
                    ]
                }
                patterns.push(pattern);
            }
        }
    }

    // dedupe 删除重复数据
    if dedupe {
        rltk::console::log(format!("Pre de-duplication, there are {} patterns", patterns.len()));
        let set:HashSet<Vec<TileType>> = patterns.drain(..).collect(); // dedup
        patterns.extend(set.into_iter());
        rltk::console::log(format!("There are {} patterns", patterns.len()));
    }
    patterns
}
// 
fn render_pattern_to_map(map : &mut Map, pattern: &Vec<TileType>, chunk_size: i32, start_x : i32, start_y: i32) {
    let mut i = 0usize;

    // 将图案 复制到地图上
    for tile_y in 0..chunk_size {
        for tile_x in 0..chunk_size {   
            let map_idx = map.xy_idx(start_x + tile_x, start_y + tile_y);
            map.tiles[map_idx] = pattern[i];
            map.visible_tiles[map_idx] = true;
        }
    }
}

// 连接性约束
pub fn patterns_to_constraints(patterns:Vec<Vec<TileType>>,chunk_size:i32)->Vec<MapChunk>{
    // move into the new constraints object
    // 将图案加到约束中
    for p in patterns{
        let mut new_chunk = MapChunk{
            pattern: p,
            exits:[Vec::new(),Vec::new(),Vec::new(),Vec::new()],
            has_exits: true,
            // 兼容
            compatible_width:[Vec::new(),Vec::new(),Vec::new(),Vec::new()],
        };

        for exit in new_chunk.exits.iter_mut() {
            for _i in 0..chunk_size{
                exit.push(false);
            }
        }

        let mut n_exits = 0;
        for x in 0..chunk_size{
            // check for north-bound exits 检查北边界线是否存在
            let north_idx = tile_idx_in_chunks(chunk_size,x,0);
            if new_chunk.pattern[north_idx] == TileType::Floor{
                new_chunk.exits[0][x as usize] =true;
                n_exits += 1;
            }

            // check for south-bound exits 检查南边界线是否存在
            let south_idx = tile_idx_in_chunks(chunk_size,x,chunk_size-1);
            if new_chunk.pattern[north_idx] == TileType::Floor{
                new_chunk.exits[0][x as usize] =true;
                n_exits += 1;
            }

            // Check for west-bound exits
            let west_idx = tile_idx_in_chunk(chunk_size, 0, x);
            if new_chunk.pattern[west_idx] == TileType::Floor {
                new_chunk.exits[2][x as usize] = true;
                n_exits += 1;
            }

            // Check for east-bound exits
            let east_idx = tile_idx_in_chunk(chunk_size, chunk_size-1, x);
            if new_chunk.pattern[east_idx] == TileType::Floor {
                new_chunk.exits[3][x as usize] = true;
                n_exits += 1;
            }

            if n_exits == 0 {
                new_chunk.has_exits = false;
            }

            constraints.push(new_chunk);
        }

        // 构建兼容性的矩阵
        let ch = constraints.clone();
        for c in constraints.iter_mut() {
            for (j,potential) in ch.iter().enumerate(){
                // if there are no exits at all, it is compatible
                if !c.has_exits || !potential.has_exits(){
                    for compat in c.compatible_width.iter_mut(){
                        compat.push(j);
                    }
                }
            }
        }
    }
}