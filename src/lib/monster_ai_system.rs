use super::{Map, Monster, Position, Viewshed};
use crate::*;
use rltk::{console, field_of_view, Point};
use specs::prelude::*;
// 怪物思考的系统 System
pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    // this data is need for system
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        // ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewshed, monster, name) = data;

        for (viewshed, _monster, name) in (&viewshed, &monster, &name).join() {
            // 如果怪物看见玩家，会做出什么行为
            if viewshed.visible_tiles.contains(&*player_pos) {
                // console::log("Monster considers their own existence"); // Web Assembly; 使用
                println!("{}shout insults!", name.name); // 常规程序使用
            }
        }
    }
}
