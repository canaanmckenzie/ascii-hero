//import rltk library
use rltk::{Rltk,GameState, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max,min};
use specs_derive::Component;

struct State{
    //ecs from spec crate
    ecs: World
}

//components

//use specs derive macro to simplify component assignment
#[derive(Component)]
struct Position {
    //2d 32 bit integers for x and y position
    x: i32,
    y: i32,
} 

//place character on screen
#[derive(Component)]
struct Renderable {
    //use glyph representation from rltk
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct LeftMover {}


#[derive(Component,Debug)]
struct Player {}



struct LeftWalker{} //empty structure to attach the system logic

//first simple system to combine entity and components into a logical framework
//implementing specs System trait for leftwalker structure
// 'a mean that the components it is using must exist as long as system is running
impl<'a> System <'a> for LeftWalker {
    //type system data tells specs what the system requires
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a,Position>);
    //fn run is a trait implementation required by impl system from specs takes itself and systemdata
    fn run(&mut self, (lefty, mut pos) : Self::SystemData){
        //system shorthand for rendering, will run for each entity that has both components leftmover and position
        for (_lefty,pos) in (&lefty, &mut pos).join() {
            //underscore means component is not used but required to have one, lefty = leftmover component
            pos.x -= 1;
            if pos.x <0 {pos.x = 79;}
        }
    }
}

//include system into state component to actually run logic
impl State {
    fn run_systems(&mut self){
        let mut lw = LeftWalker{}; //makes mutable instance of leftwalker system
        lw.run_now(&self.ecs); //tells system to run and how to find the ecs
        self.ecs.maintain(); //tells specs that if any changes were queued by sytem they should apply to world now
    }
}


impl GameState for State{
    fn tick(&mut self, ctx: &mut Rltk){
        ctx.cls();

        //call run systems for each tick in gamestate
        self.run_systems();

        player_input(self,ctx);

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
    //join both entities so that only entities with both position and player components will be operated on
    for (_player,pos) in (&mut players, &mut positions).join() {
        //check if you havent left screen - magic numbers
        pos.x = min(79, max(0,pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
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


fn main() -> rltk::BError {

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
    .with_title("Ascii Hero")
    .build()?;

    let mut gs = State {
        ecs: World::new() //method in world that creates world but does not reference itself 
    };

    //register components into created world - uses specs crate
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
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

    //add random entities
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position{x:i*7,y:20})
            .with(Renderable{
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover{})
            .build();
    }

rltk::main_loop(context,gs)
}
    