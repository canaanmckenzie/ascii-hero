//import rltk library
use rltk::{Rltk,GameState, RGB};
use specs::prelude::*;

//modules
mod map;
pub use map::*;
mod components;
pub use components::*;
mod rect;
pub use rect::Rect;
mod player;
pub use player::*;


pub struct State{
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


//main
fn main() -> rltk::BError {

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
    .with_title("Ascii Hero")
    .build()?;

    let mut gs = State {
        ecs: World::new() //method in world that creates world but does not reference itself 
    };

    //gs.ecs.insert(new_map_rooms_and_corridors()); //add randomly generated map to world - using specs shared resources all ecs can use
    //register components into created world - uses specs crate
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let (rooms, map) =  new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x,player_y) = rooms[0].center();

    //entity creation
    gs.ecs//begin method chaining
        .create_entity()
        .with(Position{x: player_x, y: player_y})
        .with(Renderable{
            glyph: rltk::to_cp437('*'),
            fg: RGB::named(rltk::PURPLE),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

rltk::main_loop(context,gs)
}
    