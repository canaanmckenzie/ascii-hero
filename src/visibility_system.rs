use specs::prelude::*;
use super::{Viewshed, Position, Map};
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl <'a>System<'a> for VisibilitySystem {
		//pass Map for use - no map is failure condition
	type SystemData = (ReadExpect<'a, Map>,
					   WriteStorage<'a, Viewshed>,
					   WriteStorage<'a, Position>);
	fn run(&mut self, data: Self::SystemData){
		let (map, mut viewshed, pos) = data;

		for (viewshed, pos) in (&mut viewshed, &pos).join(){
			viewshed.visible_tiles.clear();
			//call rltk field of view function from rltk::Point system use &*map to dereference then reference to unwrap Map from ecs
			viewshed.visible_tiles = field_of_view(Point::new(pos.x,pos.y),viewshed.range,&*map);
			//use vectors retarin function - lambda (iteratres over vector passing p as param)
			//runs every frame and stores a list of tiles as long as inside map
			viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

		}
	}
}