
use rltk::{ RGB, Rltk, Console };
use specs::prelude::*;
use super::{CombatStats, Player};
// 使用rltk specs
// 不能光看，开始要写
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