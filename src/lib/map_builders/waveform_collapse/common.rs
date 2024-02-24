use super::TileType;


#[derive(PartialEq, Eq, Hash, Clone)]
pub struct MapChunk{
    pub pattern: Vec<TileType>, //
    pub exits:[Vec<bool>;4],
    pub has_exits:bool,
    pub compatible_width:[Vec<usize>;4],
}
// tile 在chunk 中的索引，类似与map 的 xy_idx
pub fn tile_idx_in_chunk(chunk_size: i32,x:i32,y:i32) ->usize{
    ((y * chunk_size) + x) as usize
}



// chunk 之间 连接性约束 需要的的 MapChunk 和 一些辅助函数
