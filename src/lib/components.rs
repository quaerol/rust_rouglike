use rltk::RGB;

use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};

use specs_derive::*;
use specs_derive::*;
// 创建组件
#[derive(Component, ConvertSaveload, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    // 渲染顺序
    pub render_order: i32,
}

// 视野的组件 玩家的敌人都有视野组件
#[derive(Component, ConvertSaveload, Clone)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>, // 使用rltk 中的Point 来描述tiles的哪些被可见
    pub range: i32,
    // 为了提高性能，只有在需要时才更新视域，添加一个标志
    pub dirty: bool,
}
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {}

// 怪物的组件 ，让怪物进行思考
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Monster {}

// 怪物应该有名字，
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Name {
    pub name: String,
}

// 玩家 和 怪物都应该有这个组件，被阻挡的TIle
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct BlocksTile {}

// 怪物和玩家的战斗数据
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

// -----------------------------------意图组件------------------------------------------------
// 攻击意图
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToMelee {
    pub target: Entity,
}

// 遭受的攻击
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct SufferDamage {
    // 遭受多个攻击
    pub amount: Vec<i32>,
}
impl SufferDamage {
    // 新的攻击，伤害值
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage {
                amount: vec![amount],
            };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

// item 定义物品的组件
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Item {}

// 药水的实际作用
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct ProvidesHealing {
    // 恢复生命的数量
    pub heal_amount: i32,
}
// backpack  是否在背包中
#[derive(Component, ConvertSaveload, Debug, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}

// 构成物品 的 基本组件
// 可以被消耗的物品
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Consumable {}

// 组件，描述范围攻击
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Ranged {
    pub range: i32,
}

// 打击损伤，造成的损伤
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct InflictsDamage {
    pub damage: i32,
}

// AoE
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct AreaOfEffect {
    // 攻击半径
    pub radius: i32,
}

// 昏迷
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Confusion {
    // 昏迷的第几个回合
    pub turns: i32,
}
// -----------------------------意图组件-------------------------------
// 想要被拾取的物品，物品是什么，被哪个拾取
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

//intent 意图组件

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<rltk::Point>,
}

// user is the owning entity, the item is the item field, and it is aimed at target - if there is one
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
}

// ---------------------------------序列化，保存 -------------------------------
// 标记类型，marker type
pub struct SerializeMe;

// Special component that exists to help serialize the game data
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: super::map::Map,
}

// -------------------------------- 让物品可以被装备 --------------------------------
// 装备插槽
#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Melee,
    Shield,
}

// 可装备的
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Equippable {
    pub slot: EquipmentSlot,
}

// 表示被装备
#[derive(Component, ConvertSaveload, Clone)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: EquipmentSlot,
}
// 想要移除的装备
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToRemoveItem {
    pub item: Entity,
}

// 装备给与战斗的加成
#[derive(Component, ConvertSaveload, Clone)]
pub struct MeleePowerBonus {
    pub power: i32,
}
#[derive(Component, ConvertSaveload, Clone)]
pub struct DefenseBonus {
    pub defense: i32,
}

// 粒子组件，位置，渲染，生命周期
#[derive(Component, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct ParticleLifetime {
    pub lifetime_ms: f32, // 粒子存活的时间
}

// 饥饿时钟组件
#[derive(Component, Serialize, Deserialize, Clone, PartialEq)]
pub enum HungerState {
    WellFed, // 营养好的
    Normal,
    Hungry,
    Starving,
}
#[derive(Component, Serialize, Deserialize, Clone, PartialEq)]
pub struct HungerClock {
    pub state: HungerState,
    pub duration: i32, // 多少间隔时间会少生命值
}

// 提供食物
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct ProvidesFood {}
