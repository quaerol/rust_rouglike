use super::{
    generate_voronoi_spawn_regions, remove_unreachable_areas_returning_most_distant, spawner, Map,
    MapBuilder, Position, TileType, SHOW_MAPGEN_VISUALIZER,
};
use rltk::RandomNumberGenerator;
use specs::prelude::*;
use std::collections::HashMap;

pub struct MazeBuilder {
    map: Map,
    starting_position: Position,
    depth: i32,
    history: Vec<Map>,
    noise_areas: HashMap<i32, Vec<usize>>,
}
impl MapBuilder for MazeBuilder {
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

impl MazeBuilder {
    pub fn new(new_depth: i32) -> MazeBuilder {
        MazeBuilder {
            map: Map::new(new_depth),
            starting_position: Position { x: 0, y: 0 },
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
        }
    }

    #[allow(clippy::map_entry)]
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        // Find a starting point; start at the middle and walk left until we find an open tile

        self.starting_position = Position { x: 2, y: 2 };
        let mut start_idx = self
            .map
            .xy_idx(self.starting_position.x, self.starting_position.y);
        self.take_snapshot();

        while self.map.tiles[start_idx] != TileType::Floor {
            self.starting_position.x -= 1;
            start_idx = self
                .map
                .xy_idx(self.starting_position.x, self.starting_position.y);
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

//  -------------------------------Actually building a maze---------------------------------
/* Maze code taken under MIT from https://github.com/cyucelen/mazeGenerator/ */

// 表示四个方向
const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

#[derive(Copy, Clone)]
struct Cell {
    // row and column define where the cell is on the map.
    row: i32,
    column: i32,
    // walls 是一个 array ，其中 bool 代表我们定义的每个方向
    walls: [bool; 4],
    // 一个布尔值，指示我们之前是否查看过该单元格。
    visited: bool,
}

impl Cell {
    // 简单的构造函数，它创建一个在每个方向都有墙壁的单元格，并且之前没有访问过
    fn new(row: i32, column: i32) -> Cell {
        Cell {
            row,
            column,
            walls: [true, true, true, true],
            visited: false,
        }
    }

    fn remove_walls(&mut self, next: &mut Cell) {
        // x 设置为 column 值，减去下一个单元格的 column 值。 列q
        let x = self.column - next.column;
        let y = self.row - next.row;

        // 如果 x 等于 1，则 next 的列必须大于我们的列值，next 单元格位于我们当前位置的右侧。所以我们拆除右边的墙。
        if x == 1 {
            self.walls[LEFT] = false;
            next.walls[RIGHT] = false;
        // 向左走 - 所以我们移除左侧的墙。
        } else if x == -1 {
            self.walls[RIGHT] = false;
            next.walls[LEFT] = false;
        } else if y == 1 {
            self.walls[TOP] = false;
            next.walls[BOTTOM] = false;
        } else if y == -1 {
            self.walls[BOTTOM] = false;
            next.walls[TOP] = false;
        }
    }
}

// 生命周期，因为 随机数的引用依赖于Grid

struct Grid<'a> {
    // defining the size of the maze.
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    // 算法使用 backtrace 进行递归回溯，以确保每个单元格都已被处理。它只是单元索引的 vector - cells 向量的索引。
    backtrace: Vec<usize>,
    // 算法使用 current 来判断我们当前正在使用哪个 Cell
    current: usize,
    rng: &'a mut RandomNumberGenerator,
}

impl<'a> Grid<'a> {
    fn new(width: i32, height: i32, rng: &mut RandomNumberGenerator) -> Grid {
        let mut grid = Grid {
            width,
            height,
            cells: Vec::new(),
            backtrace: Vec::new(),
            current: 0,
            rng,
        };
        //
        for row in 0..height {
            for column in 0..width {
                // 它迭代网格的行和列，将新的 Cell 结构推送到 cells 向量，并按其位置进行编号。
                grid.cells.push(Cell::new(row, column));
            }
        }

        grid
    }
    // 获得单元格的数组索引，与 map 的 xy_idx 函数非常相似
    fn calculate_index(&self, row: i32, column: i32) -> i32 {
        // 边界检查
        if row < 0 || column < 0 || column > self.width - 1 || row > self.height - 1 {
            -1
        } else {
            column + (row * self.width)
        }
    }

    // 此函数提供 current 单元格的可用exit 索引 ，对任何单元格地址调用此函数都会返回一个 vector
    fn get_available_neighbors(&self) -> Vec<usize> {
        let mut neighbors: Vec<usize> = Vec::new();

        // 获取当前单元格的 row 和 column 坐标
        let current_row = self.cells[self.current].row;
        let current_column = self.cells[self.current].column;

        // 得到邻居的索引
        let neighbor_indices: [i32; 4] = [
            self.calculate_index(current_row - 1, current_column),
            self.calculate_index(current_row, current_column + 1),
            self.calculate_index(current_row + 1, current_column),
            self.calculate_index(current_row, current_column - 1),
        ];

        for i in neighbor_indices.iter() {
            if *i != -1 && !self.cells[*i as usize].visited {
                neighbors.push(*i as usize);
            }
        }

        neighbors
    }

    // 当前单元格可能无处可去 - 在这种情况下它返回 None 。否则，它返回 Some 以及下一个目标的数组索引
    fn find_next_cell(&mut self) -> Option<usize> {
        // Obtain a list of neighbors for the current cell.
        let neighbors = self.get_available_neighbors();
        if !neighbors.is_empty() {
            if neighbors.len() == 1 {
                return Some(neighbors[0]);
            } else {
                return Some(
                    neighbors[(self.rng.roll_dice(1, neighbors.len() as i32) - 1) as usize],
                );
            }
        }
        None
    }

    fn generate_maze(&mut self, generator: &mut MazeBuilder) {
        // 多久进行一次快照
        let mut i = 0;
        loop {
            // 当前单元格已经被访问
            self.cells[self.current].visited = true;
            let next = self.find_next_cell();

            match next {
                Some(next) => {
                    self.cells[next].visited = true;
                    // 将当前单元格推进递归
                    self.backtrace.push(self.current);
                    //   __lower_part__      __higher_part_
                    //   /            \      /            \
                    // --------cell1------ | cell2-----------
                    // 将单元格拆分为两个可变引用。我们需要对同一个切片进行两个可变引用

                    let (lower_part, higher_part) =
                        self.cells.split_at_mut(std::cmp::max(self.current, next));
                    // 从第一部分获取对索引较低的单元格的可变引用，从第二部分开始获取对第二个索引的单元格的可变引用。
                    let cell1 = &mut lower_part[std::cmp::min(self.current, next)];
                    let cell2 = &mut higher_part[0];
                    // 修改cell 的墙壁，开凿迷宫通道
                    cell1.remove_walls(cell2);

                    // 与上面代码功能相同, 使用指针实现
                    /*  unsafe {
                        // 下一个单元
                        // 什么类型的指针
                        let next_cell: *mut Cell = &mut self.cells[next];
                        let current_cell = &mut self.cells[self.current];
                        current_cell.remove_walls(next_cell);
                    } */
                    self.current = next;
                }
                None => {
                    // 如果 backtrace 不为空，我们将 current 设置为 backtrace 列表中的第一个值。
                    if !self.backtrace.is_empty() {
                        self.current = self.backtrace[0];
                        self.backtrace.remove(0);
                    } else {
                        // 如果 backtrace 为空，我们就完成了 - 所以我们 break 退出循环。
                        break;
                    }
                }
            }
            if i % 50 == 0 {
                self.copy_to_map(&mut generator.map);
                generator.take_snapshot();
            }
            i += 1;
        }
    }

    // 复制一份地图
    fn copy_to_map(&self, map: &mut Map) {
        // Clear the map 将地图全部变为墙
        for i in map.tiles.iter_mut() {
            *i = TileType::Wall;
        }
        // 这就是 Grid/Cell 和我们的地图格式之间的不匹配问题得到解决的地方：迷宫结构中的每个 Cell 都可以在四个主要方向中的任何一个方向上有墙。
        // 我们的地图不是这样工作的：墙不是图块的一部分，它们就是图块。因此，我们将 Grid 的大小加倍，并在不存在墙壁的地方编写雕刻地板。让我们看一下这个函数：
        for cell in self.cells.iter() {
            let x = cell.column + 1;
            let y = cell.row + 1;
            let idx = map.xy_idx(x * 2, y * 2);

            map.tiles[idx] = TileType::Floor;
            // 如果我们引用的 Cell 没有 TOP 墙，我们会将 idx 图块上方的地图图块设置为地板
            if !cell.walls[TOP] {
                map.tiles[idx - map.width as usize] = TileType::Floor
            }
            if !cell.walls[RIGHT] {
                map.tiles[idx + 1] = TileType::Floor
            }
            if !cell.walls[BOTTOM] {
                map.tiles[idx + map.width as usize] = TileType::Floor
            }
            if !cell.walls[LEFT] {
                map.tiles[idx - 1] = TileType::Floor
            }
        }
    }
}
