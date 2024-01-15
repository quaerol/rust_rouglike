use super::{Map, Monster, Position, Viewshed};
use crate::*;
use rltk::{console, field_of_view, Point};
use specs::prelude::*;
// 怪物思考的系统 System
pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    // this data is need for system
    type SystemData = (
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, pos, monster) = data;

        for (viewshed, pos, monster) in (&viewshed, &pos, &monster).join() {
            // console::log("Monster considers their own existence"); // Web Assembly; 使用
            println!("Monster considers their own existence"); // 常规程序使用
        }
    }
}
