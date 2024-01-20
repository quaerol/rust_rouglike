use crate::{GameLog, Name, Player};

use super::{CombatStats, SufferDamage};
use specs::prelude::*;

// 该系统来计算伤害值
pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;
        for (mut stats, damage) in (&mut stats, &damage).join() {
            // 使用迭代器 dui damage 进行求和
            stats.hp -= damage.amount.iter().sum::<i32>();
        }
        damage.clear();
    }
}
// add a method to clean up dead entities 删除 实体
pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    // 使用作用域来让 借用检查 高兴
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        let names = ecs.read_storage::<Name>();
        let entities = ecs.entities();
        let mut log = ecs.write_resource::<GameLog>();
        for (entity, stats) in (&entities, &combat_stats).join() {
            // 得到玩家
            let player = players.get(entity);
            match player {
                None => {
                    let victim_name = names.get(entity);
                    if let Some(victim_name) = victim_name {
                        log.entries.push(format!("{} is dead", &victim_name.name));
                    }
                    dead.push(entity)
                }
                Some(_) => println!("you are dead"),
            }
        }
    }
    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}
