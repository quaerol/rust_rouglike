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

        // 拾取 物品后 
        for pickup in wants_pickup.join(){
            // 移除position 组件
            positions.remove(pickup.item);
            // 插入InBackpack 组件
            backpack.insert(pickup.item, InBackpack{ owner: pickup.collected_by }).expect("Unable to insert backpack entry");

            if pickup.collected_by == *player_entity {
                // 打印拾取
                gamelog.entries.push(format!("You pick up the {}.", names.get(pickup.item).unwrap().name));
            }
        }
        wants_pickup.clear();
    }
}