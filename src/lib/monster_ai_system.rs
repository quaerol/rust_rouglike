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
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>, // 获得有这组件的实体
        WriteStorage<'a, WantsToMelee>,
        // 需要修改，所以是Write
        WriteStorage<'a, Confusion>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            player_entity,
            runstate,
            entities,
            mut viewshed,
            monster,
            mut position,
            mut wants_to_melee,
            mut confused,
        ) = data;
        // monsterAI system 只有在 Mons特人Trun怪物游戏状态才可以运行

        if *runstate != RunState::MonsterTurn {
            return;
        }

        for (entity, mut viewshed, _monster, mut pos) in
            (&entities, &mut viewshed, &monster, &mut position).join()
        {
            // 怪物 能否 行动 的标志
            let mut can_act = true;
            // 得到 confused 的怪物
            let is_confused = confused.get_mut(entity);
            if let Some(i_anm_confused) = is_confused {
                i_anm_confused.turns -= 1;
                if i_anm_confused.turns < 1 {
                    confused.remove(entity);
                }
                can_act = true;
            }

            if can_act {
                // 怪物和玩家的距离，怪物靠近玩家 开始 大喊大叫 并且攻击
                let distance =
                    rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance < 1.5 {
                    // Attack goes here
                    // println!("{}shout insults!", entity.name); // 常规程序使用
                    wants_to_melee
                        .insert(
                            entity,
                            WantsToMelee {
                                target: *player_entity,
                            },
                        )
                        .expect("Unable to insert attack");
                }
                // 如果怪物看见玩家，会做出什么行为
                else if viewshed.visible_tiles.contains(&*player_pos) {
                    // 玩家 和 怪物之间的路径
                    let path = rltk::a_star_search(
                        map.xy_idx(pos.x, pos.y) as i32,
                        map.xy_idx(player_pos.x, player_pos.y) as i32,
                        &mut *map,
                    );
                    // 将怪物移动到该位置
                    // `steps` is a vector of each step towards the target, *including* the starting position.
                    if path.success && path.steps.len() > 1 {
                        let mut idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = false;

                        pos.x = path.steps[1] as i32 % map.width; // steps[1] 是玩家的位置
                        pos.y = path.steps[1] as i32 / map.width;

                        idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = true;
                        viewshed.dirty = true;
                    }
                }
            }
        }
    }
}
