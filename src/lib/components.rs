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

// 玩家 和 怪物都应该有这个组件，被阻挡的TIle
#[derive(Component, Debug)]
pub struct BlocksTile {}

// 怪物和玩家的战斗数据
#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}
