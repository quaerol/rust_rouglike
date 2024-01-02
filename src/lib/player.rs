#[derive(Component)]
struct Player {}
// *****system*********************

// LeftWalker system
struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    // trait 也是需要的数据的，trait 的数据是哪来的的，来自她正在实现的struct
    type System = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>); // 移动需要修改位置的数据
    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

// player move
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Palyer>();

    let map = ecs.fetch::<vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let des_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[des_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}
