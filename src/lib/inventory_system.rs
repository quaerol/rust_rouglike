use crate::*;

use super::{gamelog::GameLog, InBackpack, Name, Position, WantsToPickupItem};
use specs::prelude::*;

// 物品菜单的状态
#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}
// 这个系统来处理物品搜集的问题
pub struct ItemCollectionSystem {}

// 记得在mian.rs 中运行 该系统
impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToPickupItem>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InBackpack>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, mut wants_pickup, mut positions, names, mut backpack) =
            data;

        // 拾取 物品后 查找 有这些组件的实体
        for pickup in wants_pickup.join() {
            // 实体移除position 组件, 该实体 移除 position 组件后, 会从地图上消失, 因为渲染的迭代的存储器 是包含 position 组件存储器的
            positions.remove(pickup.item);
            // 插入InBackpack 组件
            backpack
                .insert(
                    pickup.item,
                    InBackpack {
                        owner: pickup.collected_by,
                    },
                )
                .expect("Unable to insert backpack entry");

            if pickup.collected_by == *player_entity {
                // 打印拾取
                gamelog.entries.push(format!(
                    "You pick up the {}.",
                    names.get(pickup.item).unwrap().name
                ));
            }
        }

        // 清楚 wants_pickup WantsToPickupItem 组件中的所有实体
        wants_pickup.clear();
    }
}

// 物品使用 系统 药水使用系统 组件，想要饮用药水
pub struct ItemUseSystem {}
impl<'a> System<'a> for ItemUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        ReadExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, WantsToUseItem>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Consumable>,
        ReadStorage<'a, ProvidesHealing>,
        ReadStorage<'a, InflictsDamage>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, AreaOfEffect>,
        WriteStorage<'a, Confusion>,
        ReadStorage<'a, Equippable>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut gamelog,
            map,
            entities,
            mut wants_use,
            names,
            consumables,
            healing,
            inflict_damage,
            mut combat_stats,
            mut suffer_damage,
            aoe,
            mut confused,
            equippable,
            mut equipped,
            mut backpack,
        ) = data;
        // 迭代所有的 WantsToDrinkPotion 的意图对象，
        for (entity, useitem) in (&entities, &wants_use).join() {
            // 物体是否使用的标志
            let mut used_item = true;

            // Targeting 存储待攻击 实体
            let mut targets: Vec<Entity> = Vec::new();
            match useitem.target {
                None => {
                    // if　there is no target, apply it to the player
                    targets.push(*player_entity);
                }
                Some(target) => {
                    // 使用的物品 是否 包含 AOE 组件
                    let area_effect = aoe.get(useitem.item);
                    match area_effect {
                        None => {
                            // Single target in tile 单体
                            let idx = map.xy_idx(target.x, target.y);
                            // 遍历 tile contain
                            for mob in map.tile_content[idx].iter() {
                                targets.push(*mob);
                            }
                        }
                        Some(area_effect) => {
                            // Aoe blast 爆炸 一个 范围
                            let mut blast_tiles =
                                rltk::field_of_view(target, area_effect.radius, &*map);
                            // Retains 保留
                            blast_tiles.retain(|p| {
                                p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height - 1
                            });
                            // 遍历 这个 范围的tile
                            for tile_idx in blast_tiles.iter() {
                                let idx = map.xy_idx(tile_idx.x, tile_idx.y);
                                for mob in map.tile_content[idx].iter() {
                                    targets.push(*mob);
                                }
                            }
                        }
                    }
                }
            }

            // if it is equippable, then we want to equip it -  and unequip whatever else in that slot
            let item_equippable = equippable.get(useitem.item);
            match item_equippable {
                None => {}
                Some(can_equip) => {
                    let target_slot = can_equip.slot;
                    let target = targets[0];

                    // Remove any items the target has in the item's slot 移除目标物品槽中的任何物品
                    let mut to_unequip: Vec<Entity> = Vec::new();
                    for (item_entity, already_equipped, name) in
                        (&entities, &equipped, &names).join()
                    {
                        // 如果 这个item_entity 被装备，将其卸下放到to_unequip 列表 中
                        if already_equipped.owner == target && already_equipped.slot == target_slot
                        {
                            to_unequip.push(item_entity);
                            if target == *player_entity {
                                gamelog.entries.push(format!("You unequip {}.", name.name));
                            }
                        }
                    }
                    // 修改to_unequip 中的实体的组件信息，表示这个实体被放到背包中
                    for item in to_unequip.iter() {
                        equipped.remove(*item);
                        backpack
                            .insert(*item, InBackpack { owner: target })
                            .expect("Unable to insert backpack entry");
                    }

                    // Wield使用 the item
                    equipped
                        .insert(
                            useitem.item,
                            Equipped {
                                owner: target,
                                slot: target_slot,
                            },
                        )
                        .expect("Unable to insert equipped component");
                    backpack.remove(useitem.item);

                    if target == *player_entity {
                        gamelog.entries.push(format!(
                            "You equip {}.",
                            names.get(useitem.item).unwrap().name
                        ));
                    }
                }
            }

            // if it heals, apply the healing
            let item_heals = healing.get(useitem.item);
            match item_heals {
                None => {}

                Some(healer) => {
                    used_item = false;
                    for target in targets.iter() {
                        // target 是一个实体，存储组件获得这个实体 对应这个组件的属性数据
                        let stats = combat_stats.get_mut(*target);
                        if let Some(stats) = stats {
                            // heals up the drinker 喝了药水的生命值 最大时生命值的最大值
                            stats.hp = i32::min(stats.max_hp, stats.hp + healer.heal_amount);
                            // 如果时玩家 喝了药水，打印日志
                            if entity == *player_entity {
                                gamelog.entries.push(format!(
                                    "you drink the {}, healing {} hp.",
                                    names.get(useitem.item).unwrap().name,
                                    healer.heal_amount
                                ));
                                used_item = true;
                            }
                        }
                    }
                }
            }

            // if it inflicts(给予) 伤害，apply it to the target cell
            // This checks to see if we have an InflictsDamage component on the item - and if it does, applies the damage to everyone in the targeted cell.
            let item_damage = inflict_damage.get(useitem.item);
            match item_damage {
                None => {}
                Some(damage) => {
                    // 是否使用used_item
                    used_item = false;
                    // 遍历
                    for mob in targets.iter() {
                        SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage);
                        if entity == *player_entity {
                            let mob_name = names.get(*mob).unwrap();
                            let item_name = names.get(useitem.item).unwrap();
                            gamelog.entries.push(format!(
                                "You use {} on {}, inflicting {} hp.",
                                item_name.name, mob_name.name, damage.damage
                            ));
                        }

                        used_item = true;
                    }
                }
            }

            // Can it pass along confusion? Note the use of scopes to escape from the borrow checker!
            let mut add_confusion = Vec::new();
            {
                let causes_confusion = confused.get(useitem.item);
                match causes_confusion {
                    None => {}
                    Some(confusion) => {
                        used_item = false;
                        for mob in targets.iter() {
                            // 加入到 confusion 向量
                            add_confusion.push((*mob, confusion.turns));
                            if entity == *player_entity {
                                let mob_name = names.get(*mob).unwrap();
                                let item_name = names.get(useitem.item).unwrap();
                                gamelog.entries.push(format!(
                                    "You use {} on {}, confusing them.",
                                    item_name.name, mob_name.name
                                ));
                            }
                        }
                    }
                }
            }
            // 被confusion 的 攻击列表的对象
            for mob in add_confusion.iter() {
                confused
                    .insert(mob.0, Confusion { turns: mob.1 })
                    .expect("Unable to insert status");
            }
        }

        wants_use.clear();
    }
}

// 丢弃物品系统
pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, WantsToDropItem>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, InBackpack>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut gamelog,
            entities,
            mut wants_drop,
            names,
            mut positions,
            mut backpack,
        ) = data;

        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let mut dropper_pos: Position = Position { x: 0, y: 0 };

            {
                // 丢弃物品后，物品显示在哪个位置
                let dropped_pos = positions.get(entity).unwrap();
                dropper_pos.x = dropped_pos.x;
                dropper_pos.y = dropped_pos.y;
            }

            // positions backpack 都是组件存储器
            positions
                .insert(
                    to_drop.item,
                    Position {
                        x: dropper_pos.x,
                        y: dropper_pos.y,
                    },
                )
                .expect("Unable to insert position");
            backpack.remove(to_drop.item);

            if entity == *player_entity {
                gamelog.entries.push(format!(
                    "You drop the {}.",
                    names.get(to_drop.item).unwrap().name
                ));
            }
        }
        // 清除 wants_drop 组件存储器
        wants_drop.clear();
    }
}

// 移除装备的系统
pub struct ItemRemoveSystem {}

impl<'a> System<'a> for ItemRemoveSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToRemoveItem>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_remove, mut equipped, mut backpack) = data;

        for (entity, to_remove) in (&entities, &wants_remove).join() {
            // 从equipped 组件存储器移除这个实体
            equipped.remove(to_remove.item);
            // 将这个实体加入到equipped 组件存储器
            backpack
                .insert(to_remove.item, InBackpack { owner: entity })
                .expect("Unable to insert backpack");
        }

        wants_remove.clear();
    }
}
