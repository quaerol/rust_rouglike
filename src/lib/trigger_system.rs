use crate::{InflictsDamage, ParticleBuilder, SingleActivation, SufferDamage};

use super::{gamelog::GameLog, EntityMoved, EntryTrigger, Hidden, Map, Name, Position};
use specs::prelude::*;

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, EntityMoved>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, EntryTrigger>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, Name>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, InflictsDamage>,
        WriteExpect<'a, ParticleBuilder>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, SingleActivation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            mut entity_moved,
            position,
            entry_trigger,
            mut hidden,
            names,
            entities,
            mut log,
            inflicts_damage,
            mut particle_builder,
            mut inflict_damage,
            single_activation,
        ) = data;

        // Iterate the entities that moved and their final position
        // iterate all entities that have a Position and EntityMoved component
        let mut remove_entities: Vec<Entity> = Vec::new();
        for (entity, mut _entity_moved, pos) in (&entities, &mut entity_moved, &position).join() {
            // obtain the map index for the location
            let idx = map.xy_idx(pos.x, pos.y);
            // iterate the tile_content index to see waht is in the new tile
            for entity_id in map.tile_content[idx].iter() {
                if entity != *entity_id {
                    // Do not bother to check yourself for being a trap!
                    //   // to see if there is a trap there
                    // if there is, we get its name and notify the player via the log that a trap activated
                    // remove the hidden component from the trap, since we know that it is there
                    let maybe_trigger = entry_trigger.get(*entity_id);
                    match maybe_trigger {
                        None => {}
                        Some(_trigger) => {
                            // We triggered it
                            let name = names.get(*entity_id);
                            if let Some(name) = name {
                                log.entries.push(format!("{} triggers!", &name.name));
                            }

                            hidden.remove(*entity_id); // The trap is no longer hidden

                            // if the trap is damage inflicating, do it
                            let damage = inflicts_damage.get(*entity_id);
                            if let Some(damage) = damage {
                                particle_builder.request(
                                    pos.x,
                                    pos.y,
                                    rltk::RGB::named(rltk::ORANGE),
                                    rltk::RGB::named(rltk::BLACK),
                                    rltk::to_cp437('‼'),
                                    200.0,
                                );
                                SufferDamage::new_damage(
                                    &mut inflict_damage,
                                    entity,
                                    damage.damage,
                                );
                            }

                            //if it is single activation, it needs to be removed
                            let sa = single_activation.get(*entity_id);
                            if let Some(_sa) = sa {
                                remove_entities.push(*entity_id);
                            }
                        }
                    }
                }
            }
        }

        // remove ant single activation traps
        for trap in remove_entities.iter() {
            entities.delete(*trap).expect("unable to delete trap");
        }

        // Remove all entity movement markers
        // 清空组件存储器，下一帧继续添加
        entity_moved.clear();
    }
}
