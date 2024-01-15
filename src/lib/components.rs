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
