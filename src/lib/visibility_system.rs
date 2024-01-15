use crate::{Map, Player};

use super::{Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::prelude::*;
// 一个系统，一个通用的视域

// 系统也需要一个struct 来表示为一个实体，
pub struct VisibilitySystem {}
// 为新系统实现system

// 向RLTK请求视域所需要的内容，Ｍap  ReadExpect<'a, Map>,
impl<'a> System<'a> for VisibilitySystem {
    // 系统需要使用的数据
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false; // 渲染可见范围后，修改标志位
                                        // first clear the list of visible tiles.
                viewshed.visible_tiles.clear();
                // &*map  先取出map 然后在使用map 引用
                // field_of_view 从 map 中计算视域
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                // retain  返回符合函数要求的
                viewshed
                    .visible_tiles
                    .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

                // if this is the player , reveal what they can see
                let _p: Option<&Player> = player.get(ent);
                if let Some(_p) = _p {
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
