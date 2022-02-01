// mod components;
mod evolution_system;
mod map;

// use components::*;
use evolution_system::Evolution;
use map::{draw, Map};
use rltk::{GameState, Rltk};
use specs::{prelude::*, RunNow, World, WorldExt};

pub const MAPWIDTH: usize = 80;
pub const MAPHEIGHT: usize = 50;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;
pub const NUMCOLORS: i32 = 10;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut evo = evolution_system::Evolution;
        evo.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        map::draw(self, ctx);
        self.run_systems();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Daemons").build()?;
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Map>();

    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    // world.insert(Map::new(&mut world));
    let map = Map::new(&mut gs.ecs);
    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
