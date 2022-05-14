use specs::prelude::*;
use super::{Viewshed, Position, Map, Player};
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl <'a>System<'a> for VisibilitySystem {
		//pass Map for use - no map is failure condition
	type SystemData = (WriteExpect<'a, Map>,
					   Entities<'a>,
					   WriteStorage<'a, Viewshed>,
					   WriteStorage<'a, Position>,
					   ReadStorage<'a, Player>);
	fn run(&mut self, data: Self::SystemData){
		let (mut map,entities, mut viewshed, pos, player) = data;

		for (ent,viewshed, pos) in (&entities, &mut viewshed, &pos).join(){
			viewshed.visible_tiles.clear();
			//call rltk field of view function from rltk::Point system use &*map to dereference then reference to unwrap Map from ecs
			viewshed.visible_tiles = field_of_view(Point::new(pos.x,pos.y),viewshed.range,&*map);
			//use vectors retarin function - lambda (iteratres over vector passing p as param)
			//runs every frame and stores a list of tiles as long as inside map
			viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

			//if entity is a player, reveal what they can see
			let p: Option<&Player> = player.get(ent);
			if let Some(p) = p {
				for vis in viewshed.visible_tiles.iter() {
					let idx = map.xy_idx(vis.x,vis.y);
					map.revealed_tiles[idx] = true;
				}
			}
		}
	}
}