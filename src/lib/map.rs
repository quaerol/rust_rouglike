// -----------------------Map section --------------------
// 地图的类型，枚举
enum TileType {
    Wall,  // “#”符号
    Floor, // “.” 符号
}
pub fn xy_idx(x: i32, y: i32) -> usize {
    todo!();
}

fn new_map() -> Vec<TileType> {
    todo!();
    // 包围的墙壁和地图内随机位置的墙
}
fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    todo!();
    // ctx.set()
}
