use super::{MapBuilder, Map, Rect, apply_room_to_map, 
    TileType, Position, spawner, SHOW_MAPGEN_VISUALIZER};
use rltk::RandomNumberGenerator;
use specs::prelude::*;

const MIN_ROOM_SIZE : i32 = 8;

pub struct CellularAutomataBuilder {
    map : Map,
    starting_position : Position,
    depth: i32,
    history: Vec<Map>,
    noise_areas : HashMap::new()
}

impl MapBuilder for CellularAutomataBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }
    fn build_map(&mut self)  {
        self.build();
    }

    fn spawn_entities(&mut self, ecs : &mut World) {
        // 从噪声区域生成实体
        for area in self.noise_areas.iter() {
            spawner::spawn_region(ecs, area.1, self.depth);
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
impl CellularAutomataBuilder {
    pub fn new(new_depth : i32) -> CellularAutomataBuilder {
        CellularAutomataBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new()
        }
    }

    fn build(&mut self){
        let mut rng = RandomNumberGenerator::new();

        // First we completely randomize the map, setting 55% of it to be floor.
        for y in 1..self.map.height-1 {
            for x in 1..self.map.width-1 {
                let roll = rng.roll_dice(1,100);
                let idx = self.map.xy_idx(x,y);
                if roll > 55{
                    self.map.tiles[idx] = TileType::Floor;
                }
                else{
                    self.map.tiles[idx] = TileType::Wall;
                }
            }

        }
    }
    self.take_snapshot();


    // now we iteratively apply cellular automata rules
    for _i in 0..15{
        let mut newtiles = self.map.tiles.clone();

        // 遍历地图
        for y in 1..self.map.height-1{
            for x in 0..self.map.width-1{
                let idx = self.map.xy_idx(x.y);
                let mut neighbors = 0;
                // 该tile 四周的邻居是否是Wall
                if self.map.tiles[idx - 1] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx + 1] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx - self.map.width as usize] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx + self.map.width as usize] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx - (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx - (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx + (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx + (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }

                // 如果该地图四周都是墙，那么该地图也是墙
                if neighbors > 4 || neighbors == 0 {
                    newtiles[idx] = TileType::Wall;
                }else{
                    newtiles[idx] = TileType::Floor;
                }
            }
        }
        // 将更新后的地图赋值给地图
        self.map.tiles = newtiles.clone();
        self.take_snapshot();

        // Find a starting point; start at the middle and walk left until we find an open tile
        self.starting_position = Position{
            x:self.map.width /2 ,y:self.map.height /2
        };
        let mut start_idx = self.map.xy_idx(self.starting_position.x,self.starting_position.y);
        while self.map.tiles[start_idx] != TileType::Floor {
            self.starting_position.x -= 1;
            start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);
        }
    }

    // find all tiles we can reach from the starting point
    // map_starts 的向量，并给它一个值：玩家开始的图块索引。 Dijkstra 地图可以有多个起点（距离 0），因此即使只有一个选择，它也必须是一个向量
    let map_starts : Vec<usize> = vec![start_idx];
    // 要求 RLTK 为我们制作一张 Dijkstra 地图。它的尺寸与主地图相匹配，使用起点，具有对地图本身的读取权限，并且我们将在 200 步时停止计数
    let dijkstra_map = rltk::DijkstraMap::new(self.map.width, self.map.height, &map_starts , &self.map, 200.0);
    // exit_tile tuple 设置为 0 和 0.0 。第一个零是出口的瓦片索引，第二个零是到出口的距离。
    let mut exit_tile = (0, 0.0f32);
    // 单元格索引添加为元组中的第一个参数。然后我们解构以获得图块和索引
    for (i,tile) in self.map.tiles.iter_mut().enumerate() {
        if *tile == TileType::Floor{
            // 从 Dijkstra 地图中获取该 tile 到起点的距离
            let distance_to_start = dijkstra_map.map[i];
            // we can not get to this tile - so we will make it a wall
            if distance_to_start == std::f32::MAX{
                *tile = TileType::Wall;
            }else{
                // if it is further away than our current exit candidate, move the exit 
                // 如果距离大于 exit_tile 元组中的距离，我们将存储新距离和新图块索引。
                if distance_to_start > exit_tile.1{
                    exit_tile.0 = i;
                    exit_tile.1 = distance_to_start;
                }
            }
        }
    }
    // 一旦我们访问了每个图块，我们就会拍摄快照以显示删除的区域。
    self.take_snapshot();
    // 将 exit_tile （最远可到达的图块）处的图块设置为向下的楼梯。
    self.map.tiles[exit_tile.0] = TileType::DownStairs;
    self.take_snapshot();

    // now we build a noise map for use in spawning entities lateer
    // create a new FastNoise object
    let mut noise = rltk::FastNoise::seeded(rng.roll_dice(1,65536) as u64);
    noise.set_noise_type(rltk::NoiseType::Cellular);
    noise.set_frequency(0.08);
    // 指定 Manhattan 距离函数
    noise.set_cellular_distance_function(rltk::CellularDistanceFunction::Manhattan);
    for y in 1 .. self.map.height-1 {
        for x in 1 .. self.map.width-1 {
            let idx = self.map.xy_idx(x, y);
            if self.map.tiles[idx] == TileType::Floor {
                // 查询 FastNoise 以获得坐标的噪声值（将它们转换为 f32 浮点数，因为库喜欢浮点数）。我们乘以 10240.0 因为默认值非常小 - 这使其达到合理的范围。
                let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.0;
                let cell_value = cell_value_f as i32;
                // 如果 noise_areas 地图包含我们刚刚生成的区域编号，我们将图块索引添加到向量中。
                if self.noise_areas.contains_key(&cell_value) {
                    self.noise_areas.get_mut(&cell_value).unwrap().push(idx);
                } else {
                    // 将创建一个新的瓦片索引向量，其中包含地图索引号。
                    self.noise_areas.insert(cell_value, vec![idx]);
                }
            }
        }
    }
}