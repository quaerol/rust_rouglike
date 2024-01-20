use super::{BlocksTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

// 这个系统负责填充 blocked list,这个blocked list 有包含BlocksTile 组件的实体，也有地图中的Wall
impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );
    fn run(&mut self, data: Self::SystemData) {
        // 通过map tile 的 content 来找到实体
        let (mut map, position, blockers, entities) = data;
        // 先将 Wall 填充进 blocked list
        map.populate_blocked();
        map.clear_content_index();
        // 得到所有实体并且得到所有的实体的position
        for (entity, position) in (&entities, &position).join() {
            let idx = map.xy_idx(position.x, position.y);
            // if they block, update the blocking list, 如何这个实体被 blocked，将其加入到 map 的 blocked 中
            let _p: Option<&BlocksTile> = blockers.get(entity);
            if let Some(_p) = _p {
                map.blocked[idx] = true;
            }
        }
    }
}
