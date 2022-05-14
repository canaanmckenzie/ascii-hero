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
mod visibility_system;
use visibility_system::VisibilitySystem;


pub struct State{
    //ecs from spec crate
    ecs: World
}

//include system into state component to actually run logic
impl State {
    fn run_systems(&mut self){
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain(); 
        //tells specs that if any changes were queued by sytem they should apply to world now
    }
}


impl GameState for State{
    fn tick(&mut self, ctx: &mut Rltk){
        ctx.cls();

        //call run systems for each tick in gamestate
        self.run_systems();

        player_input(self,ctx);

        //fetch will crash if resource doesnt exist
        draw_map(&self.ecs,ctx);

        //ask read access to container used to store position/render components
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        //specs uses a database like join to find entities that have both position and renderable components
        //destructuring in rust, on result per entity that has both components
        //returns a tuple {} of entities with components pos and render .o and .1
        //entities with one or the other will not be included 

        for (pos, render) in (&positions, &renderables).join(){
            let idx = map.xy_idx(pos.x,pos.y);
            if map.visible_tiles[idx]{

                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
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
        ecs: World::new() 
        //method in world that creates world but does not reference itself 
    };

    //register components into created world - uses specs crate
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map: Map =  Map::new_map_rooms_and_corridors();
    let (player_x,player_y) = map.rooms[0].center();

    //monster spawner
    let mut rng = rltk::RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1){
        let (x,y) = room.center();

        let glyph: rltk::FontCharType;
        let roll = rng.roll_dice(1,2);
        match roll {
            1 =>{glyph = rltk::to_cp437('g')}
            _ =>{glyph = rltk::to_cp437('o')}
        }

        //entity
        gs.ecs.create_entity()
        //components
            .with(Position{x,y})
            .with(Renderable{
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
            .build();
    }
    gs.ecs.insert(map);

    //entity creation
    gs.ecs//begin method chaining
        .create_entity()
        .with(Position{x: player_x, y: player_y})
        .with(Renderable{
            glyph: rltk::to_cp437('☼'),
            fg: RGB::named(rltk::BLUE),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
        .build();


rltk::main_loop(context,gs)
}
    