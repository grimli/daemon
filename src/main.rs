// mod components;
mod evolution_system;
mod map;

// use components::*;
use evolution_system::Evolution;
use map::Map;
use specs::{prelude::*, RunNow, World, WorldExt};

pub const MAPWIDTH: usize = 80;
pub const MAPHEIGHT: usize = 43;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;
pub const NUMCOLORS: i32 = 10;

fn main() {
    let mut world = World::new();
    // world.register::<Position>();
    // world.register::<Color>();
    world.register::<Map>();

    world.insert(rltk::RandomNumberGenerator::new());

    // world.insert(Map::new(&mut world));
    let map = Map::new(&mut world);
    world.insert(map);

    let mut evo = Evolution;
    evo.run_now(&world);

    world.maintain();
}
