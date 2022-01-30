use super::Position;
use specs::prelude::*;

pub struct Evolution;

impl<'a> System<'a> for Evolution {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}
