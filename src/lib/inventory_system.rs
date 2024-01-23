use crate::{CombatStats, Potion, WantsToDrinkPotion, WantsToDropItem};

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

<<<<<<< HEAD
// 物品使用 系统 药水使用系统 组件，想要饮用药水
pub struct ItemUseSystem{}
impl<'a> System<'a> for ItemUseSystem{
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        ReadExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToDrinkPotion>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Consumable>,
                        ReadStorage<'a, ProvidesHealing>,
                        ReadStorage<'a, InflictsDamage>,
                        WriteStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>,
                        ReadStorage<'a, AreaOfEffect>,
                        WriteStorage<'a, Confusion>
                      );
    fn run(&mut self,data:Self::SystemData){
        let (player_entity, mut gamelog, map, entities, mut wants_use, names,
            consumables, healing, inflict_damage, mut combat_stats, mut suffer_damage,
            aoe, mut confused) = data;
=======
// 药水使用系统 组件，想要饮用药水
pub struct PotionUseSystem {}
impl<'a> System<'a> for PotionUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, WantsToDrinkPotion>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Potion>,
        WriteStorage<'a, CombatStats>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut gamelog,
            entities,
            mut wants_drink,
            names,
            potions,
            mut combat_stats,
        ) = data;
>>>>>>> 9e4331d0c4be8600a52fbda944a5d70201688752

        // 迭代所有的 WantsToDrinkPotion 的意图对象，
        for (entity, useitem) in (&entities, &wants_use).join() {
            // the potion 想要被喝
<<<<<<< HEAD
            let item_heals = healing.get(useitem.item);
            match item_heals {
                None => {},
                Some(healer) =>{
                    // heals up the drinker 喝了药水的生命值 最大时生命值的最大值 
                    stats.hp = i32::min(stats.max_hp,stats.hp +  healer.heal_amount);
                    // 如果时玩家 喝了药水，打印日志
                    if entity == *player_entity{
                        gamelog.entities.push(format!("you drink the {}, healing {} hp.",names.get(useitem.item).unwarp().name,healer.heal_amount));
=======
            let potion = potions.get(drink.potion);
            match potion {
                None => {}
                Some(potion) => {
                    // heals up the drinker 喝了药水的生命值 最大时生命值的最大值
                    stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);
                    // 如果时玩家 喝了药水，打印日志
                    if entity == *player_entity {
                        gamelog.entries.push(format!(
                            "you drink the {}, healing {} hp.",
                            names.get(drink.potion).unwrap().name,
                            potion.heal_amount
                        ));
>>>>>>> 9e4331d0c4be8600a52fbda944a5d70201688752
                    }
                    // 删除 the potion 将该实体标记为 dead 但是不会在系统中删除他们，
                    // 检查是否有 消耗组件
                    let consumable = consumables.get(useitem.item);
                    match consumable {
                        None => {}
                        Some(_) => {
                            entities.delete(useitem.item).expect("Delete failed");
                        }
                    }
                }
            }
        }

        wants_drink.clear();
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
