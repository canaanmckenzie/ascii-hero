//import rltk library
use rltk::{Rltk,GameState, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max,min};
use specs_derive::Component;

//modules
mod map;
pub use map::*;
mod components;
pub use components::*;
mod rect;
pub use rect::Rect;


struct State{
    //ecs from spec crate
    ecs: World
}

//include system into state component to actually run logic
impl State {
    fn run_systems(&mut self){
        self.ecs.maintain(); //tells specs that if any changes were queued by sytem they should apply to world now
    }
}


impl GameState for State{
    fn tick(&mut self, ctx: &mut Rltk){
        ctx.cls();

        //call run systems for each tick in gamestate
        self.run_systems();

        player_input(self,ctx);

        //fetch will crash if resource doesnt exist
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map,ctx);

        //ask read access to container used to store position/render components
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        //specs uses a database like join to find entities that have both position and renderable components
        //destructuring in rust, on result per entity that has both components
        //returns a tuple {} of entities with components pos and render .o and .1
        //entities with one or the other will not be included 
        for (pos, render) in (&positions, &renderables).join(){
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

//control functions - player
fn try_move_player(delta_x:i32,delta_y:i32, ecs: &mut World){
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
fn player_input(gs: &mut State, ctx: &mut Rltk){
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



//main
fn main() -> rltk::BError {

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
    .with_title("Ascii Hero")
    .build()?;

    let mut gs = State {
        ecs: World::new() //method in world that creates world but does not reference itself 
    };

    gs.ecs.insert(new_map_rooms_and_corridors()); //add randomly generated map to world - using specs shared resources all ecs can use
    //register components into created world - uses specs crate
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();


    //entity creation
    gs.ecs//begin method chaining
        .create_entity()
        .with(Position{x:40, y:25})
        .with(Renderable{
            glyph: rltk::to_cp437('*'),
            fg: RGB::named(rltk::PURPLE),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

rltk::main_loop(context,gs)
}
    