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
struct Position{
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

impl GameState for State{
    fn tick(&mut self, ctx: &mut Rltk){
        ctx.cls();
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
    gs.ecs//begin method chaining
        .create_entity()
        .with(Position{x:40, y:25})
        .with(Renderable{
            glyph: rltk::to_cp437('*'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
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
            .build();
    }

rltk::main_loop(context,gs)
}
    
