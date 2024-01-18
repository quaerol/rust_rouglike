use rltk::RGB;
use specs::Component;
use specs::DenseVecStorage;
use specs_derive::Component;
// 创建组件
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

// 视野的组件 玩家的敌人都有视野组件
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>, // 使用rltk 中的Point 来描述tiles的哪些被可见
    pub range: i32,
    // 为了提高性能，只有在需要时才更新视域，添加一个标志
    pub dirty: bool,
}
#[derive(Component)]
pub struct Player {}

// 怪物的组件 ，让怪物进行思考
#[derive(Component, Debug)]
pub struct Monster {}

// 怪物应该有名字，
#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

// potion 药剂，生命药水

#[derive(Component, Debug)]
pub struct Item {}

#[derive(Component, Debug)]
pub struct Potion {
    // 恢复生命的数量
    pub heal_amount : i32
}

// pcking up item 表示这个实体是否在 背包中
#[derive(Component, Debug, Clone)]
pub struct InBackpack {
    pub owner : Entity
}

// -----------------------------意图组件-------------------------------
// 想要被拾取的物品，物品是什么，被哪个拾取
#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
    pub collected_by : Entity,
    pub item : Entity
}

// 使用药水的意图组件
#[derive(Component, Debug, Clone)]
pub struct WantsToDrinkPotion[
    pub potion: Entity
]

// 丢弃 物品的意图组件
#[derive(Component, Debug, Clone)]
pub struct WantsToDropItem{
    pub item : Entity
}
