use super::{
    generate_voronoi_spawn_regions, remove_unreachable_areas_returning_most_distant, spawner, Map,
    MapBuilder, Position, TileType, SHOW_MAPGEN_VISUALIZER,
};
use rltk::RandomNumberGenerator;
use specs::prelude::*;
// 没有将 pub 放在这些前面：在当前模块使用, 不会将它们暴露在模块之外。这有助于我们保持代码干净，并缩短编译时间！
mod image_loader;
use image_loader::*;

pub struct WaveformCollapseBuilder {
    map: Map,
    starting_position: Position,
    depth: i32,
    history: Vec<Map>,
    noise_areas: HashMap<i32, Vec<usize>>,
}

impl MapBuilder for WaveformCollapseBuilder {
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

impl WaveformCollapseBuilder {
    pub fn new(new_depth: i32) -> WaveformCollapseBuilder {
        WaveformCollapseBuilder {
            map: Map::new(new_depth),
            starting_position: Position { x: 0, y: 0 },
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
        }
    }

    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        const CHUNK_SIZE :i32 = 7;

        // 将xp 图像文件转为 map
        self.map = load_rex_map(
            self.depth,
            &rltk::rex::XpFile::from_resource("../../resources/wfc-demo1.xp").unwrap(),
        );
        self.take_snapshot();

        // 构架图案，然后将图案绘制到地图上
        let patterns = build_patterns(&self.map, CHUNK_SIZE, true, true);
        self.render_tile_gallery(&patterns, CHUNK_SIZE);
        // chunk 之间的连接性约束,
        let constraints = patterns_to_constraints(pattern,CHUNK_SIZE);

        // Find a starting point; start at the middle and walk left until we find an open tile
        self.starting_position = Position {
            x: self.map.width / 2,
            y: self.map.height / 2,
        };
        let mut start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);
        /*while self.map.tiles[start_idx] != TileType::Floor {
            self.starting_position.x -= 1;
            start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);
        }*/
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

    // display our tiles as part of the snapshot system
    // gallery 走廊
    fn render_tile_gallery(&mut self,patterns: &Vec<Vec<TileType>>,chunk_size: i32){
        self.map = Map::new(0);

        let mut counter = 0;
        let mut x = 1;
        let mut y = 1;
        // 一个一个将图案渲染到地图上
        while counter < patterns.len() {
            render_pattern_to_map(&mut self.map,&patterns[counter],chunk_size,x,y);

            x += chunk_size +1;
            // 一行行的渲染
            if x+chunk_size > self.map.width{
                // move to the next row
                x = 1;
                y += chunk_size+1;
                if y + chunk_size > self.map.height{
                    // move to the next page
                    self.take_snapshot();
                    self.map = Map::new();

                    x =1;
                    y =1;
                }
            }
            // 表示复制图案到地图上数量加1
            counter +=1;
        }
        // 拍摄快照
        self.take_snapshot();
    }
}
