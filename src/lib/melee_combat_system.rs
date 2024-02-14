use super::{
    gamelog::GameLog, particle_system::ParticleBuilder, CombatStats, DefenseBonus, Equipped,
    MeleePowerBonus, Name, Position, SufferDamage, WantsToMelee,
};
use specs::prelude::*;

// 该系统 来 处理近战
pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        // 装备的战斗攻击和防御加成
        ReadStorage<'a, MeleePowerBonus>,
        ReadStorage<'a, DefenseBonus>,
        ReadStorage<'a, Equipped>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Position>,
    );
    fn run(&mut self, data: Self::SystemData) {
        // destruct data 结构 data
        let (
            entities,
            mut log,
            mut wants_melee,
            names,
            combat_stats,
            mut inflict_damage,
            melee_power_bonuses,
            defense_bonuses,
            equipped,
            mut particle_builder,
            positions,
        ) = data;

        for (entity, wants_melee, name, stats) in
            (&entities, &wants_melee, &names, &combat_stats).join()
        {
            // Once we've determined that the attacker is alive, we set offensive_bonus to 0. offensive 攻击
            if stats.hp > 0 {
                let mut offensive_bonus = 0;
                // we iterate all entities that have a MeleePowerBonus and Equipped entry, if they are equipped by attacker, we add their power bonus to offensive_bonus
                for (_item_entity, power_bonus, equipped_by) in
                    (&entities, &melee_power_bonuses, &equipped).join()
                {
                    if equipped_by.owner == entity {
                        offensive_bonus += power_bonus.power;
                    }
                }
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                // Once we have determined that the defender is alive, we set defensive_bonus to 0.
                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();

                    let mut defensive_bonus = 0;
                    // We iterate all entities that have a DefenseBonus and an Equipped entry. If they are equipped by the target, we add their defense to the defense_bonus.
                    for (_item_entity, defense_bonus, equipped_by) in
                        (&entities, &defense_bonuses, &equipped).join()
                    {
                        if equipped_by.owner == wants_melee.target {
                            defensive_bonus += defense_bonus.defense;
                        }
                    }
                    let pos = positions.get(wants_melee.target);
                    //
                    if let Some(pos) = pos {
                        particle_builder.request(
                            pos.x,
                            pos.y,
                            rltk::RGB::named(rltk::ORANGE),
                            rltk::RGB::named(rltk::BLACK),
                            rltk::to_cp437('‼'),
                            200.0,
                        );
                    }

                    // When we calculate damage, we add the offense bonus to the power side - and add the defense bonus to the defense side.
                    let damage = i32::max(
                        0,
                        (stats.power + offensive_bonus) - (target_stats.defense + defensive_bonus),
                    );

                    if damage == 0 {
                        log.entries.push(format!(
                            "{} is unable to hurt {}",
                            &name.name, &target_name.name
                        ));
                    } else {
                        log.entries.push(format!(
                            "{} hits {}, for {} hp.",
                            &name.name, &target_name.name, damage
                        ));
                        SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                    }
                }
            }
        }

        wants_melee.clear();
    }
}
