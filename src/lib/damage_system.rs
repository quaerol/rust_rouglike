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
        for (mut stats, damage) in (&mut stats, &damage) {
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
        let entities = ecs.entities();
        for (entity, stats) in (&entities, &combat_stats).join() {
            // hp 小于1 将其加入到 死亡列表
            if stats.hp < 1 {
                dead.push(entity);
            }
        }
    }
    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}
