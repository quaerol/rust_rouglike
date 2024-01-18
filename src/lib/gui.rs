
use rltk::{ RGB, Rltk, Console };
use specs::prelude::*;
use super::{CombatStats, Player};

// 不能光看，开始要写

// 物品菜单的状态
#[derive(PartialEq, Copy,Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Seleted,
}


// 绘制UI
pub fn draw_ui(ecs:World,ctx:&mut Rltk){
    ctx.draw_box(0, 43, 79, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    // 在 ui 中显示 玩家的生命值信息
    // 得到有CombatStates 和 Player 组件的实体
    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    // 遍历这两个都有的实体
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(12, 43, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &health);

        // hp bar 的 位置
        ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, RGB::named(rltk::RED), RGB::named(rltk::BLACK));
    }
    // 在 ui 中打印日志
    let log = ecs.fetch::<GameLog>();
    let mut y = 44;
    for s in log.entries.iter().rev(){
        if y < 49 {
            ctx.print(2,y,s);
        }
        y += 1;
    }
    // 在MeleeCombatSystem 混战系统中打印 攻击日志
    // in delete_the_dead 中 打印 死亡日志


    // 获取鼠标的支持和工具提示
    // draw mouse cursor ，鼠标的指向的位置颜色为洋红色
    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0,mouse_pos.1,RGB::named(rltk::MAGENTA));

    // 画出 工具提示的支持
    draw_tooltips();
}

// 画出工具提示
fn draw_tooltips(ecs:&World,ctx:&mut Rltk){
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height{
        return;
    }

    let mut tooltip:Vec<String> = Vec::new();
    // 有name 和 position 的实体都可以 显示 提示
    for (name, position) in (&name, &position).join(){
        // 将实体的 position 坐标 变为 索引 idx 
        let idx = map.xy_idx(position.x, position.y);
        // 如果实体的位置和鼠标的位置相同，说明鼠标点击了实体，并且实体在mao 上是可见的
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            // 把实体的名字记录在工具提示中
            tooltip.push(name.name.to_string());
        }
    }

    if !tooltip.is_empty() {
        let mut width:i32 = 0;
        for s in tooltip.iter(){
            // 根据 实体的名字的长度，自动调节显示框的大小
            if width < s.len() as i32{
                width = s.len() as i32;
            }
        }

        width += 3;

        // 鼠标在左侧，提示显示在右边
        if mouse_pos.0 > 40{
            let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;   // 想左偏移 
            
            let mut y = mouse_pos.1;
            for s in tooltip.iter(){
                ctx.print_color(left_x, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), s);
                let padding = (width - s.len() as i32)-1;
                for i in 0..padding {
                    ctx.print_color(arrow_pos.x - i, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &"->".to_string());
        }else{
            let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 +3;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x + 1, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), s);
                let padding = (width - s.len() as i32)-1;
                for i in 0..padding {
                    ctx.print_color(arrow_pos.x + 1 + i, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &"<-".to_string());
        }
    }
}

// 显示库存
pub fn show_inventory(gs:&mut State,ctx:&mut Rltk) -> ItemMenuResult{
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs>read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpacks>();
    let entities = gs.ecs.entities();

    // 闭包过滤
    let inventory = (&backpacks, &names).join().filter(|item| item.0.owner == *player_entity);
    // 菜单的大小和物品数量相关 计算 背包中的所有的物品
    let count = inventory.count();

    let mut y = (25 - (count - 2)) as i32;
    // 绘制 菜单 框 和 标题 和 描述
    ctx.draw_box(15, y-2, 31, (count+3) as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color(18, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Inventory");
    ctx.print_color(18, y+count as i32+1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "ESCAPE to cancel");

    // 可以装备的物品, 一个物品 就一个可以装配
    let mut equippable: Vec<Entity> = Vec::new();
    // 物品的序号 (a) (b) (c)
    let mut j = 0;

    // 过滤 物品的所有者 是 player 
    for (entity, _pack, name) in (&entities, &backpack, &names).join().filter(|item| item.1.owner == *player_entity ) {
        // 设置显示的物品的序号
        ctx.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
        ctx.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as rltk::FontCharType);
        ctx.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437(')'));

        // 物品的名字  j 是 物品的序号 y 是物品渲染的位置
        ctx.print(21,y,&name.name.to_string());
        equippable.push(entity);

        y += 1;
        j += 1;

    }

    // 匹配  Rltk 中的 键
    match ctx.key{
        None => ItemMenuResult::NoResponse,
        Some(key) => {
            match key{
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel,None)}
                _ => {
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count as i32 {
                        // 得到 对应按键的物品 并返回
                        return (ItemMenuResult::Selected, Some(equippable[selection as usize])); 
                    }
                    (ItemMenuResult::NoResponse,None)
                }
            }
        }
    }
}

// 删除项目的 菜单
pub fn drop_item_menu(gs:&mut State,ctx:&mut Rltk)->(ItemMenuResult, Option<Entity>){
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names).join().filter(|item| item.0.owner == *player_entity );
    // 统计库存
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(15, y-2, 31, (count+3) as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color(18, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Drop Which Item?");
    ctx.print_color(18, y+count as i32+1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "ESCAPE to cancel");

    let mut equippable : Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack, &names).join().filter(|item| item.1.owner == *player_entity ) {
        ctx.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
        // a is ASCII 97
        ctx.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as rltk::FontCharType);
        ctx.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
        j += 1;
    }

    // 匹配按键
    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
                _ => { 
                    //  得到选中物品的按键
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count as i32 {
                        return (ItemMenuResult::Selected, Some(equippable[selection as usize]));
                    }  
                    (ItemMenuResult::NoResponse, None)
                }
            }
        }
    }
}
