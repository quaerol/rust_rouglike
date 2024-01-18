// 生成怪物 和 物品
use rltk::{ RGB, RandomNumberGenerator };
use specs::prelude::*;
use super::{CombatStats, Player, Renderable, Name, Position, Viewshed, Monster, BlocksTile};

const MAX_MONSTERS:i32 = 4;
const MAX_ITEMS:i32 = 2;

pub fn spawn_room(ecs:&mut World,room:&Rect){
    let mut monster_spawn_points:Vec<usize> = Vec::new();
    let mut item_spawn_points:Vec<usize> = Vec::new();


    // scope to keep the borrow checker happy
    {
        // 随机的数量
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
        let num_items = rng.roll_dice(1, MAX_ITEMS + 2) - 3;

        for _i in 0 .. num_monsters {
            let mut added = false;
            while !added {
                // 房间中的随机的位置
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                // 不能在一个位置重复spawn 
                if !monster_spawn_points.contains_key(&idx) {
                    monster_spawn_points.push(idx);
                    added = true; 
                }
            }
            // 添加物品
            for _i in 0 .. num_items {
                let mut added = false;
                while !added {
                    let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                    let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                    let idx = (y * MAPWIDTH) + x;
                    if !item_spawn_points.contains(&idx) {
                        item_spawn_points.push(idx);
                        added = true;
                    }
                }
            }
        }
    }
    // actually spawn the monster
    for idx in monster_spawn_points.iter(){
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_monster(ecs,x as i32,y as i32);
    }
    // actually spawn the monster
    for idx in item_spawn_points.iter(){
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        health_potion(ecs,x as i32,y as i32);
    }

}
/// spawn the player and return his/her entity object
pub fn player(ecs:&mut World,player_x:i32,player_y:i32)->Entity{
    ecs
        .create_entity()
        // with 各种组件
        .with(Position{x:player_x, y:player_y})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .with(Name{name: "Player".to_string() })
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5 })
        .build()
}

// spawns a random monster at a given location
pub fn random_monster(ecs:&mut World,x:i32,y:i32){
    let roll:i32;
    {
        // 灵活使用作用域，释放一些临时使用的数据
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1,2);
    }
    match roll{
        1 => {orc(ecs,x,y)}
        _ => {goblin(ecs,x,y)}
    }
}

fn orc(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, rltk::to_cp437('o'), "Orc"); }
fn goblin(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, rltk::to_cp437('g'), "Goblin"); }

// 泛型 S 实现呢ToString trait
fn monster<S:ToString>(ecs: &mut World, x: i32, y: i32,glyph: rltk::FontCharType,name:S){
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .with(Monster{})
        .with(Name{ name : name.to_string() })
        .with(BlocksTile{})
        .with(CombatStats{ max_hp: 16, hp: 16, defense: 1, power: 4 })
        .build();
}

// 然后在main.rs 中使用该模块的生成函数，创建 玩家 和 怪物

// spawn health_potion, 在地图中创建 恢复药剂实体，需要世界和创建的位置
fn health_potion(ecs:&mut World,x:i32,y:i32){
    ecs.create_entity()
        .with(Position{x,y})
        .with(Renderable{
            glyph: rltk::to_cp437('¡'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Name{ name : "Health Potion".to_string() })
        .with(Item{})
        .with(Potion{ heal_amount: 8 }) // 恢复生命值的数量
        .build();
}