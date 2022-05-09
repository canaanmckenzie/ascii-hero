use rltk::{RGB,Rltk,RandomNumberGenerator};
use std::cmp::{max,min};


//map tiles using enumeration - derive features are built into rust
//clone adds a .clone() method to type copy makes replica does not move object
#[derive(PartialEq,Copy,Clone)]
pub enum TileType {
    Wall, Floor
}


//environment functions
//find index of array for entire map
pub fn xy_idx(x: i32, y:i32) -> usize{
    //returns basic size usize 
    //lacks semicolon means implied return statement
    (y as usize * 80) + x as usize
}


//map constructor function
pub fn new_map() -> Vec<TileType> {
    //changeable variable map - vec! is a procedural macro that runs like a function, could write fn to add
    let mut map = vec![TileType::Floor; 80 * 50];

    //make boundaries walls
    for x in 0..80 {
        map[xy_idx(x,0)] = TileType::Wall;
        map[xy_idx(x,49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0,y)] = TileType::Wall;
        map[xy_idx(79,y)] = TileType::Wall;
    }

    //randomly put walls in - wont be nice but its a start
    let mut rng = rltk::RandomNumberGenerator::new();

    //not looking for i so _i, just want it to run 400 times
    for _i in 0..400 {
        let x = rng.roll_dice(1,79);
        let y = rng.roll_dice(1,49);
        let idx = xy_idx(x,y);
        if idx != xy_idx(40,25){
            map[idx] = TileType::Wall;
        }
    }

    map
}


  //[Tiletype] passes in slices 
pub fn draw_map(map: &[TileType], ctx: &mut Rltk){
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter(){
        //render tile depending on the type
        match tile {
            TileType::Floor => {
                ctx.set(x,y,RGB::from_f32(0.5,0.5,0.5),RGB::from_f32(0.,0.,0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x,y,RGB::from_f32(0.,1.0,0.), RGB::from_f32(0.,0.,0.), rltk::to_cp437('#'));
            }
        }

        //move coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
