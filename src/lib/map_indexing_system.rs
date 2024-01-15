use super::{BlocksTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

// 这个系统负责填充 blocked list,这个blocked list 有包含BlocksTile 组件的实体，也有地图中的Wall
impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers) = data;
        // 先将 Wall 填充进 blocked list
        map.populate_blocked();
        // 将有BlocksTile 组件的实体 填充进去,同时有 Position AND BlocksTile 组件的实体
        for (position, _blocks) in (&position, &blockers).join() {
            let idx = map.xy_idx(position.x, position.y);
            map.blocked[idx] = true;
        }
    }
}
