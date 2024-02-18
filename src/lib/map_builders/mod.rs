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
    // Note that until we have a second map type, this isn't even slighlty random
    // This is actually two calls, now: we make a box with Box::new(...), and we place an empty SimpleMapBuilder into the box.
    Box::new(SimpleMapBuilder::new(new_depth))
}

