use super::components;
use super::map;
use super::State;
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

/// moves the player
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<components::Position>();
    let mut players = ecs.write_storage::<components::Player>();
    let map = ecs.fetch::<Vec<map::TileType>>();

    for (_player, position) in (&mut players, &mut positions).join() {
        let destination_idx = map::xy_idx(position.x + delta_x, position.y + delta_y);
        if map[destination_idx] != map::TileType::Wall {
            position.x = min(79, max(0, position.x + delta_x));
            position.y = min(49, max(0, position.y + delta_y));
        }
    }
}

/// accepts input from keyboard arrow keys
pub fn player_input(game_state: &mut State, ctx: &Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut game_state.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut game_state.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut game_state.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut game_state.ecs),
            _ => {}
        },
    }
}
