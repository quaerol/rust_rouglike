use rltk::rex::XpFile;

// 素材文件
rltk::embedded_resource!(SMALL_DUNGEON, "../../resources/SmallDungeon_80x50.xp");

pub struct RexAssets {
    pub menu: XpFile,
}
impl RexAssets {
    pub fn new() -> RexAssets {
        rltk::link_resource!(SMALL_DUNGEON, "../../resources/SmallDungeon_80x50.xp");

        RexAssets {
            // loads the Rex paint file from memory.
            menu: XpFile::from_resource("../../resources/SmallDungeon_80x50.xp").unwrap(),
        }
    }
}
