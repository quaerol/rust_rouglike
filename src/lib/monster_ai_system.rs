use super::{Map, Monster, Position, Viewshed};
use crate::*;
use rltk::{console, field_of_view, Point};
use specs::prelude::*;
// 怪物思考的系统 System
pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    // this data is need for system
    #[allow(clippy::type_complexity)] // 允许在在一个类型中同时使用这么多的类型
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>, // Point是资源 表示玩家所在的位置
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>, // 获得有这组件的实体
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;

        for (mut viewshed, _monster, name, mut pos) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            // 怪物和玩家的距离，怪物靠近玩家 开始 大喊大叫
            let distance =
                rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                // Attack goes here
                // console::log(&format!("{} shouts insults", name.name));
                println!("{}shout insults!", name.name); // 常规程序使用
                return;
            }
            // 如果怪物看见玩家，会做出什么行为
            if viewshed.visible_tiles.contains(&*player_pos) {
                // 玩家 和 怪物之间的路径
                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );
                // 将怪物移动到该位置
                // `steps` is a vector of each step towards the target, *including* the starting position.
                if path.success && path.steps.len() > 1 {
                    pos.x = path.steps[1] as i32 % map.width; // steps[1] 是玩家的位置
                    pos.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
