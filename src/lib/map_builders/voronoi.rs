use super::{MapBuilder, Map,
    TileType, Position, spawner, SHOW_MAPGEN_VISUALIZER,
    remove_unreachable_areas_returning_most_distant, generate_voronoi_spawn_regions};
use rltk::RandomNumberGenerator;
use specs::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum DistanceAlgorithm { Pythagoras, Manhattan, Chebyshev }

pub struct VoronoiCellBuilder {
    map : Map,
    starting_position : Position,
    depth: i32,
    history: Vec<Map>,
    noise_areas : HashMap<i32, Vec<usize>>,
    // 种子的数量
    n_seeds: usize,
    // 距离算法
    distance_algorithm: DistanceAlgorithm
}
impl MapBuilder for VoronoiCellBuilder {
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

impl VoronoiCellBuilder {
    #[allow(dead_code)]
    pub fn new(new_depth : i32) -> VoronoiCellBuilder {
        VoronoiCellBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new(),
            n_seeds: 64,
            distance_algorithm: DistanceAlgorithm::Pythagoras
        }
    }
    // 不同的算法，不同的地图类型
    pub fn pythagoras(new_depth : i32) -> VoronoiCellBuilder {
        VoronoiCellBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new(),
            n_seeds: 64,
            distance_algorithm: DistanceAlgorithm::Pythagoras
        }
    }
    
    pub fn manhattan(new_depth : i32) -> VoronoiCellBuilder {
        VoronoiCellBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new(),
            n_seeds: 64,
            distance_algorithm: DistanceAlgorithm::Manhattan
        }
    }

    #[allow(clippy::map_entry)]
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        // Make a Voronoi diagram. We'll do this the hard way to learn about the technique!
        let mut voronoi_seeds : Vec<(usize, rltk::Point)> = Vec::new();

        while voronoi_seeds.len() < self.n_seeds {
            // 种子的随机位置
            let vx = rng.roll_dice(1,self.map_width-1);
            let vy = rng.roll_dice(1,self.map_height-1);

            let vidx = self.map.xy_idx(vx, vy);
            let candidate = (vidx, rltk::Point::new(vx, vy));
            if !voronoi_seeds.contains(&candidate) {
                voronoi_seeds.push(candidate);
            }
        }

        // 下一步是确定每个单元的 Voronoi 成员资格
        let mut voronoi_distance = vec![(0, 0.0f32) ; n_seeds];
        // 每个图块都被赋予 Voronoi 组的成员资格，该组的种子在物理上最接近。
        // 使用它来存储图块属于哪个 Voronoi 单元。
        let mut voronoi_membership : Vec<i32> = vec![0 ; self.map.width as usize * self.map.height as usize];

        for (i,vid) in voronoi_membership.iter_mut().enumerate() {
            // 将索引转为地图上的坐标
            let x = i as i32 % self.map.width;
            let y = i as i32 / self.map.width;
            // 种子的索引和地图上的坐标
            for (seed,pos) in voronoi_seeds.iter().enumerate() {
                // 地图上的图块的坐标和种子的位置之间的距离
                let distance;
                match self.distance_algorithm {      
                    DistanceAlgorithm::Pythagoras =>{    
                        distance = rltk::DistanceAlg::PythagorasSquared.distance2d(
                            rltk::Point::new(x, y), 
                            pos.1
                        );
                    }
                    // 这就是曼哈顿距离的作用：它像曼哈顿出租车司机一样计算距离 - 行数加列数，而不是直线距离。
                    DistanceAlgorithm::Manhattan => {
                        distance = rltk::DistanceAlg::Manhattan.distance2d(
                            rltk::Point::new(x, y), 
                            pos.1
                        );
                    }
                    DistanceAlgorithm::Chebyshev => {
                        distance = rltk::DistanceAlg::Chebyshev.distance2d(
                            rltk::Point::new(x, y), 
                            pos.1
                        );
                    }
                }
                // voronoi_distance[seed] 设置为 种子索引 和 该种子和地图上各个tile距离。
                voronoi_distance[seed] = (seed, distance);
            }
            // 根据距离进行排序，因此最接近的该种子的tile
            voronoi_distance.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
            // 图块的 vid （Voronoi ID）
            // 种子的索引index存储在 Voronoi 成员组中, 直接修改 voronoi_membership 数据
            *vid = voronoi_distance[0].0 as i32;
        }
        // You can summarize that in English more easily:
        // each tile is given membership of the Voronoi group to whom's seed it is physically closest.
        // 每个图块得到一个最接近他的种子
        // 绘制地图
        for y in 1..self.map.height-1 {
            for x in 1..self.map.width-1 {
                let mut neighbors = 0;
                let my_idx = self.map.xy_idx(x, y);
                // 计算不同 Voronoi 组中有多少相邻图块
                let my_seed = voronoi_membership[my_idx];
                if voronoi_membership[self.map.xy_idx(x-1, y)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x+1, y)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x, y-1)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x, y+1)] != my_seed { neighbors += 1; }
                // 如果答案是 0，那么它完全在组中：所以我们可以放置一个地板。
                // 如果答案为 1，则它仅与其他 1 个组接壤 - 因此我们还可以放置一个地板（以确保我们可以在地图上行走）
                if neighbors < 2 {
                    self.map.tiles[my_idx] = TileType::Floor;
                }
            }
            self.take_snapshot();
        }

        // Find a starting point; start at the middle and walk left until we find an open tile
        self.starting_position = Position{ x: self.map.width / 2, y : self.map.height / 2 };
        let mut start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);
        while self.map.tiles[start_idx] != TileType::Floor {
            self.starting_position.x -= 1;
            start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);
        }
        self.take_snapshot();

        // Find all tiles we can reach from the starting point
        let exit_tile = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);
        self.take_snapshot();

        // Place the stairs
        self.map.tiles[exit_tile] = TileType::DownStairs;
        self.take_snapshot();

        // Now we build a noise map for use in spawning entities later
        self.noise_areas = generate_voronoi_spawn_regions(&self.map, &mut rng);
    }
}