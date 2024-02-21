use super::{MapBuilder, Map,  
    TileType, Position, spawner, SHOW_MAPGEN_VISUALIZER};
use rltk::RandomNumberGenerator;
use specs::prelude::*;
use std::collections::HashMap;


#[derive(PartialEq, Copy, Clone)]
pub enum DrunkSpawnMode { StartingPoint, Random }

pub struct DrunkardSettings {
    // 不同的醉汉生成模式
    pub spawn_mode : DrunkSpawnMode
    // 醉汉昏倒的时间
    pub drunken_lifetime : i32,
    pub floor_percent: f32
}



pub struct DrunkardsWalkBuilder {
    map : Map,
    starting_position : Position,
    depth: i32,
    history: Vec<Map>,
    noise_areas : HashMap<i32, Vec<usize>>,
    settings : DrunkardSettings
}

impl MapBuilder for DrunkardsWalkBuilder {
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
impl DrunkardsWalkBuilder {
    pub fn new(new_depth : i32,settins:DrunkardSettings) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new(),
            settings
        }
    }
    // 预设一些醉汉漫步的模式
    pub fn open_area(new_depth) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new(),
            settings:DrunkardSettings{
                spawn_mode: DrunkSpawnMode::StartingPoint,
                drunken_lifetime: 400,
                floor_percent: 0.5
            }
        }
    }
    pub fn open_halls(new_depth) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new(),
            settings:DrunkardSettings{
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 400,
                floor_percent: 0.5
            }
        }
    }
    pub fn winding_passages(new_depth) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new(),
            settings:DrunkardSettings{
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4
            }
        }
    }
    

    #[allow(clippy::map_entry)]
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

         // Set a central starting point
        self.starting_position = Position{ x: self.map.width / 2, y: self.map.height / 2 };
        let start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);

        self.map.tiles[start_idx] = TileType::Floor;

        let total_tiles = self.map.width * self.map.height;
        // 希望达到的floor数量
        let desired_floor_tiles = let desired_floor_tiles = (self.settings.floor_percent * total_tiles as f32) as usize;

        let mut floor_tile_count = self.map.tiles.iter().filter(|a| **a == TileType::floor).count();
        let mut digger_count = 0;
        // 挖掘的活动次数
        let mut active_digger_count = 0;

        // 如果地图floor 的数量小于 想要达到的floor数量，醉汉继续找地图
        while floor_tile_count < desired_floor_tiles{
            let mut did_something = false;
            let mut drunk_x;
            let mut drunk_y;
            // 改变醉酒者漫步的模式, 开始漫步的起始点在哪里
            match self.settings.spawn_mode{
                DrunkSpawnMode::StartingPoint => {
                    drunk_x = self.starting_position.x;
                    drunk_y = self.starting_position.y;
                }
                // 如果我们处于“随机”模式，醉汉的起始位置是第一个挖掘者的地图中心（以确保楼梯周围有一些空间），然后是随机的每次后续迭代的地图位置。
                DrunkSpawnMode::Random => {
                    // 第一次挖掘
                    if digger_count == 0 {
                        drunk_x = self.starting_position.x;
                        drunk_y = self.starting_position.y;
                    } else {
                        drunk_x = rng.roll_dice(1, self.map.width - 3) + 1;
                        drunk_y = rng.roll_dice(1, self.map.height - 3) + 1;
                    }
                }
            }
            let mut drunk_life = self.settings.drunken_lifetime;

            
            while drunk_life > 0 {
                let drunk_idx = self.map.xy_idx(drunk_x, drunk_y);
                // 如果醉汉碰到的tile 是 Wall ,将这个墙挖开，变为Floor
                if self.map.tiles[drunk_idx] == TileType::Wall {
                    did_something = true;
                }
                // 用DownStairs; 表示该符号是
                self.map.tiles[drunk_idx] = TileType::DownStairs;

                // 醉汉下一步的方向
                let stagger_direction = rng.roll_dice(1.4);
                match stagger_direction {
                    // 1 向左移动
                    1 => { if drunk_x > 2 { drunk_x -= 1; } }
                    2 => { if drunk_x < self.map.width-2 { drunk_x += 1; } }
                    3 => { if drunk_y > 2 { drunk_y -=1; } }
                    _ => { if drunk_y < self.map.height-2 { drunk_y += 1; } }
                }
                // 移动一次，生命值减一
                drunk_life -= 1;

            }
            if did_something { 
                // 拍一个快照
                self.take_snapshot(); 
                active_digger_count += 1;
            }
            digger_count += 1;
            // 
            for t in self.map.tiles.iter_mut() {
                if *t == TileType::DownStairs {
                    *t = TileType::Floor;
                }
            }
            floor_tile_count = self.map.tiles.iter().filter(|a| **a == TileType::Floor).count();
        }

        rltk::console::log(format!("{} dwarves gave up their sobriety, of whom {} actually found a wall.", digger_count, active_digger_count));




        // Find all tiles we can reach from the starting point
        let exit_tile = remove_unreachable_areas_returning_most_distant(&mut self.map,start_idx);
         
        self.take_snapshot();

        // Place the stairs
        self.map.tiles[exit_tile] = TileType::DownStairs;
        self.take_snapshot();

        // now we build a noise map for use in spawning entities later
        self.noise_areas = generate_voronoi_spawn_regions(&self.map,rng);

    }
}

