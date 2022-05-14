use rltk::{RGB,Rltk,RandomNumberGenerator, BaseMap, Algorithm2D, Point};
use std::cmp::{max,min};
use super::{Rect,Viewshed, Player};
use specs::prelude::*;

//map tiles using enumeration - derive features are built into rust
//clone adds a .clone() method to type copy makes replica does not move object
#[derive(PartialEq,Copy,Clone)]
pub enum TileType {
    Wall, Floor
}

#[derive(Default)]
pub struct Map{
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>
}

//2d system from rltk
impl Algorithm2D for Map {
    //return rltk point from dimensions functions passing in Map params
    fn dimensions(&self) -> Point {
        Point::new(self.width,self.height)
    }
}

//basemap system from rltk
impl BaseMap for Map {
    //returns bool if tiles are a wall - can't see through walls from rltk
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
}


//implement a system for map structure for variabilty later
impl Map {
    //environment functions
    //find index of array for entire map
    pub fn xy_idx(&self, x: i32, y:i32) -> usize{
        //returns basic size usize 
        //lacks semicolon means implied return statement
        (y as usize * self.width as usize) + x as usize
    }


    fn apply_room_to_map(&mut self, room: &Rect){
        for y in room.y1 + 1..= room.y2{
            for x in room.x1 + 1..= room.x2{
                let idx = self.xy_idx(x,y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    //reference mutable map structure-system of Map as map
    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32){
        for x in min(x1,x2)..=max(x1,x2){
            let idx = self.xy_idx(x,y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32){
        for y in min(y1,y2)..=max(y1,y2){
            let idx = self.xy_idx(x,y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }


    pub fn new_map_rooms_and_corridors() -> Map {

        //create a new map structure to reference and pass into environment
    let mut map = Map{
        tiles: vec![TileType::Wall; 80*50],
        rooms: Vec::new(),
        width: 80,
        height: 50,
        revealed_tiles: vec![false;80*50]
    };

    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE,MAX_SIZE);
        let h = rng.range(MIN_SIZE,MAX_SIZE);
        let x = rng.roll_dice(1,map.width - w - 1) - 1;
        let y = rng.roll_dice(1, map.height - h - 1) - 1;
        let new_room = Rect::new(x,y,w,h);
        let mut ok = true;
        for other_room in map.rooms.iter() {
            if new_room.intersect(other_room) {ok = false }
        } if ok {
            map.apply_room_to_map(&new_room);
            if !map.rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();

                if rng.range(0,2)==1 {
                    map.apply_horizontal_tunnel( prev_x,new_x,prev_y);
                    map.apply_vertical_tunnel(prev_y,new_y,new_x);
                } else {
                    map.apply_vertical_tunnel(prev_y,new_y,prev_x);
                    map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                }
            }
            map.rooms.push(new_room);
        }
    }

    map
    }
}
    //pass in a world component from specs::prelude::*
  //[Tiletype] passes in slices 
pub fn draw_map(ecs: &World, ctx: &mut Rltk){

    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;

    for (idx,tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
                match tile {
                    TileType::Floor => {
                        ctx.set(x,y,RGB::from_f32(0.5,0.5,0.5),RGB::from_f32(0.,0.,0.), rltk::to_cp437('▒'));
                    }
                    TileType::Wall => {
                        ctx.set(x,y,RGB::from_f32(0.1,0.1,0.1), RGB::from_f32(0.,0.,0.), rltk::to_cp437('#'));
                    }
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
