mod components;
mod evolution_system;

use components::*;
use evolution_system::Evolution;
use specs::{prelude::*, ReadStorage, RunNow, System, VecStorage, World, WorldExt};

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Color>();

    let point = world
        .create_entity()
        .with(Position { x: 4, y: 7 })
        .with(Color { value: 0 })
        .build();

    let mut hello_world = Evolution;
    hello_world.run_now(&world);
    world.maintain();
}
