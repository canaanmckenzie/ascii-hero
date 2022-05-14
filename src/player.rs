use rltk::{Rltk,VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{min,max};
use super::*;



//control functions - player
pub fn try_move_player(delta_x:i32,delta_y:i32, ecs: &mut World){
    //gains write access to position and player
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    //join both entities so that only entities with both position and player components will be operated on
    for (_player,pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y +delta_y);
        //check if you havent left screen - magic numbers - check if there is no wall
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0,pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty =  true;
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
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::A => try_move_player(-1,0, &mut gs.ecs),
            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::D => try_move_player(1,0, &mut gs.ecs),
            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::W => try_move_player(0,-1, &mut gs.ecs),
            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::S => try_move_player(0,1, &mut gs.ecs),
            _=> {} //anything else
        },
    }
}

