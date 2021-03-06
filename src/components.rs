use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};


//use specs derive macro to simplify component assignment
#[derive(Component)]
pub struct Position {
    //2d 32 bit integers for x and y position
   pub x: i32,
   pub y: i32,
} 

//place character on screen
#[derive(Component)]
pub struct Renderable {
    //use glyph representation from rltk
   pub glyph: rltk::FontCharType,
   pub fg: RGB,
   pub bg: RGB,
}


#[derive(Component,Debug)]
pub struct Player {}


#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool
}

#[derive(Component)]
pub struct Monster {}

#[derive(Component,Debug)]
pub struct Name {
    pub name: String
}

#[derive(Component,Debug)]
pub struct BlocksTile {}