use rltk::{Rltk,VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{min,max};
use super::*;



//control functions - player
pub fn try_move_player(delta_x:i32,delta_y:i32, ecs: &mut World){
    //gains write access to position and player
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    //join both entities so that only entities with both position and player components will be operated on
    for (_player,pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y +delta_y);
        //check if you havent left screen - magic numbers - check if there is no wall
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0,pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

//read keyboard provided by rltk
pub fn player_input(gs: &mut State, ctx: &mut Rltk){
    //player movement
    //context provides information about a key
    // match avoids need to unwrap because key stored as an option type
    match ctx.key { 
        //option types none or some provided by rust
        None => {} //nothing happened
        //key variable from ctx from Rltk is an enumeration - hold variable from predefined values
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1,0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1,0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0,-1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0,1, &mut gs.ecs),
            _=> {} //anything else
        },
    }
}

