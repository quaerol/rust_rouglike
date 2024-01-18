use specs::prelude::*;
use super::{WantsToPickupItem, Name, InBackpack, Position, gamelog::GameLog};

// 这个系统来处理物品搜集的问题
pub struct ItemCollectionSystem {}

// 记得在mian.rs 中运行 该系统
impl<'a> System<'a> for ItemCollectionSystem{
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, WantsToPickupItem>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, InBackpack>
                      );
    fn run(&mut self,data:Self::SystemData){
        let (player_entity, mut gamelog, mut wants_pickup, mut positions, names, mut backpack) = data;

        // 拾取 物品后 查找 有这些组件的实体
        for pickup in wants_pickup.join(){
            // 移除position 组件
            positions.remove(pickup.item);
            // 插入，将其分配给InBackpack 组件
            backpack.insert(pickup.item, InBackpack{ owner: pickup.collected_by }).expect("Unable to insert backpack entry");

            if pickup.collected_by == *player_entity {
                // 打印拾取
                gamelog.entries.push(format!("You pick up the {}.", names.get(pickup.item).unwrap().name));
            }
        }

        // 清楚 wants_pickup WantsToPickupItem 组件中的所有实体
        wants_pickup.clear();
    }
}

// 药水使用系统 组件，想要饮用药水
pub struct PotionUseSystem{}
impl<'a> System<'a> for PotionUseSystem{
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToDrinkPotion>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Potion>,
                        WriteStorage<'a, CombatStats>
                      );
    fn run(&mut self,data:Self::SystemData){
        let (player_entity, mut gamelog, entities, mut wants_drink, names, potions, mut combat_stats) = data;

        // 迭代所有的 WantsToDrinkPotion 的意图对象，
        for (entity, drink, stats) in (&entities, &wants_drink, &mut combat_stats).join() {
            // the potion 想要被喝
            let potion = potions.get(drink.potion);
            match potion {
                None => {},
                Some(potion) =>{
                    // heals up the drinker 喝了药水的生命值 最大时生命值的最大值 
                    stats.hp = i32::min(stats.max_hp,stats.hp +  potion.heal_amount);
                    // 如果时玩家 喝了药水，打印日志
                    if entity == *player_entity{
                        gamelog.entities.push(format!("you drink the {}, healing {} hp.",name.get(drink.potion).unwarp().name,potion.heal_amount));
                    }
                    // 删除 the potion 将该实体标记为 dead 但是不会在系统中删除他们，
                    entities.delete(drink.potion).expect("Delete failed");
                }
            }
        }

        wants_drink.clear();
    }
}

pub struct ItemDropSytem{}

impl<'a> System<'a> for ItemDropSytem{
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToDropItem>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, InBackpack>
                      );
    fn run(&mut self, data : Self::SystemData) {
        let (player_entity, mut gamelog, entities, mut wants_drop, names, mut positions, mut backpack) = data;

        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let mut dropper_pos: Position = Position{x:0,y:0};

            {
                // 丢弃物品后，物品显示在哪个位置
                let dropped_pos = position.get(entity).unwarp();
                dropper_pos.x = dropped_pos.x;
                dropper_pos.y = dropped_pos.y;
            }

            // positions backpack 都是组件存储器
            positions.insert(to_drop.item, Position{ x : dropper_pos.x, y : dropper_pos.y }).expect("Unable to insert position");
            backpack.remove(to_drop.item);

            if entity == *placed_entity{
                gamelog.entries.push(format!("You drop the {}.", names.get(to_drop.item).unwrap().name));
            }

        }
        // 清除 wants_drop 组件存储器
        wants_drop.clear();
    }
}