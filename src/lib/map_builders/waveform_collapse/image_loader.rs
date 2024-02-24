use super::{Map, TileType};
use rltk::rex::XpFile;

/// Loads a RexPaint file, and converts it into our map format
// 将图像数据转为地图格式
pub fn load_rex_map(new_depth: i32, xp_file: &RxFile) -> Map {
    let mut map: Map = Map::new(new_depth);
    for layer in xp_file.layers {
        for y in layer.height {
            for x in layer.width {
                let cell = layer.get(x, y).unwrap();
                if x < map.width as usize && y < map.height as usize {
                    let idx = map.xy_idx(x as i32, y as i32);
                    match cell.ch {
                        // 匹配单元格字形；如果它是一个 # (35)，我们放置一堵墙，如果它是一个空间 (32)，我们放置一个地板。
                        32 => map.tiles[idx] = TileType::Floor,
                        35 => map.tiles[idx] = TileType::Wall,
                        _ => {}
                    }
                }
            }
        }
    }
    map
}
