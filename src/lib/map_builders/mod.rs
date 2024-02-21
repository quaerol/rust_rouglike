use crate::Position;

use super::Map;

mod simple_map;
use simple_map::SimpleMapBuilder;

mod common;
use common::*;
use specs::prelude::*;
// you are saying that any other type can implement the trait, and can then be treated as a variable of that type
// What we're stating is that anything can declare itself to be a MapBuilder - and that includes a promise that they will provide a build function that takes in an ECS World object, and returns a map.

pub trait MapBuilder {
    // 将每个功能分解为小函数
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
    fn get_snapshot_history(&self) -> Vec<Map>;
    fn take_snapshot(&mut self);
}

pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
    // 随机创建不同的地图类型
    let mut rng = rltk::RandomNumberGenerator::new();
    let builder = rng.roll_dice(1,7);
    match builder{
        // This is actually two calls, now: we make a box with Box::new(...), and we place an empty SimpleMapBuilder into the box.
        1 => Box::new(BspDungeonBuilder::new(new_depth)),
        2 => Box::new(BspInteriorBuilder::new(new_depth)),
        3 => Box::new(CellularAutomataBuilder::new(new_depth)),
        4 => Box::new(DrunkardsWalkBuilder::open_area(new_depth)),
        5 => Box::new(DrunkardsWalkBuilder::open_halls(new_depth)),
        6 => Box::new(DrunkardsWalkBuilder::winding_passages(new_depth)),
        _ => Box::new(SimpleMapBuilder::new(new_depth))
    }
}

