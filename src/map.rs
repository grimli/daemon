use super::{World, MAPCOUNT, MAPHEIGHT, MAPWIDTH, NUMCOLORS};
use rltk::RandomNumberGenerator;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Default, Component, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub range: i32,
    pub states: Vec<i32>,
}

impl Map {
    /// Generates a generic empty map
    pub fn new(world: &mut World) -> Map {
        let mut rng = world.write_resource::<RandomNumberGenerator>();
        let mut map = Map {
            width: MAPWIDTH,
            height: MAPHEIGHT,
            range: NUMCOLORS,
            states: Vec::new(),
        };

        for idx in 0..(MAPCOUNT - 1) {
            map.states.push(rng.roll_dice(1, NUMCOLORS));
        }
        map
    }

    pub fn down(&self, idx: usize) -> i32 {
        let new_idx = idx + self.width;
        if new_idx < self.width * self.height {
            self.states[new_idx]
        } else {
            self.states[new_idx % self.width]
        }
    }

    pub fn draw(&self) {}

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = (idx % self.width) as i32;
        let y = (idx / self.width) as i32;
        (x, y)
    }

    pub fn left(&self, idx: usize) -> i32 {
        if (idx) % self.width == 0 {
            self.states[idx + self.width - 1]
        } else {
            self.states[idx - 1]
        }
    }

    pub fn right(&self, idx: usize) -> i32 {
        if (idx + 1) % self.width == 0 {
            self.states[idx - self.width + 1]
        } else {
            self.states[idx + 1]
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn up(&self, idx: usize) -> i32 {
        if idx < self.width {
            let new_idx = self.width * (self.height - 1) + idx;
            self.states[new_idx]
        } else {
            let new_idx = idx - self.width;
            self.states[new_idx]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Map, MAPHEIGHT, MAPWIDTH};
    use specs::{prelude::*, World};

    #[test]
    fn idx_xy_test() {
        let mut world = World::new();
        world.insert(rltk::RandomNumberGenerator::new());
        let map = Map::new(&mut world);

        let res = map.idx_xy(3 * map.width + 3);
        assert_eq!(res, (3, 3));
    }

    #[test]
    fn xy_idx_test() {
        let mut world = World::new();
        world.insert(rltk::RandomNumberGenerator::new());

        let map = Map::new(&mut world);
        assert_eq!((3 * map.width + 2) as usize, map.xy_idx(2, 3));
    }

    #[test]
    fn down_standard() {
        let mut world = World::new();
        world.insert(rltk::RandomNumberGenerator::new());
        let mut map = Map::new(&mut world);
        for idx in 0..(map.width * map.height - 1) {
            map.states[idx as usize] = 1
        }
        map.states[2 * map.width + 5] = 2;
        assert_eq!(map.down(map.width + 5), 2)
    }

    #[test]
    fn down_border() {
        let mut world = World::new();
        world.insert(rltk::RandomNumberGenerator::new());
        let mut map = Map::new(&mut world);
        for idx in 0..(map.width * map.height - 1) {
            map.states[idx as usize] = 1
        }
        map.states[5] = 2;
        assert_eq!(map.down(map.width * (map.height - 1) + 5), 2)
    }

    #[test]
    fn up_standard() {
        let mut world = World::new();
        world.insert(rltk::RandomNumberGenerator::new());
        let mut map = Map::new(&mut world);
        for idx in 0..(map.width * map.height - 1) {
            map.states[idx as usize] = 1
        }
        map.states[map.width + 5] = 2;
        assert_eq!(map.up(2 * map.width + 5), 2)
    }

    #[test]
    fn up_border() {
        let mut world = World::new();
        world.insert(rltk::RandomNumberGenerator::new());
        let mut map = Map::new(&mut world);
        for idx in 0..(map.width * map.height - 1) {
            map.states[idx as usize] = 1
        }
        map.states[(map.height - 1) * map.width + 5] = 2;
        assert_eq!(map.up(5), 2)
    }

    #[test]
    fn left_standard() {
        let mut world = World::new();
        world.insert(rltk::RandomNumberGenerator::new());
        let mut map = Map::new(&mut world);
        for idx in 0..(map.width * map.height - 1) {
            map.states[idx as usize] = 1
        }
        map.states[2 * map.width + 5] = 2;
        assert_eq!(map.left(2 * map.width + 6), 2)
    }

    #[test]
    fn left_border() {
        let mut world = World::new();
        world.insert(rltk::RandomNumberGenerator::new());
        let mut map = Map::new(&mut world);
        for idx in 0..(map.width * map.height - 1) {
            map.states[idx as usize] = 1
        }
        map.states[3 * map.width - 1] = 2;
        assert_eq!(map.left(2 * map.width), 2)
    }

    #[test]
    fn right_standard() {
        let mut world = World::new();
        world.insert(rltk::RandomNumberGenerator::new());
        let mut map = Map::new(&mut world);
        for idx in 0..(map.width * map.height - 1) {
            map.states[idx as usize] = 1
        }
        map.states[2 * map.width + 5] = 2;
        assert_eq!(map.right(2 * map.width + 4), 2)
    }

    #[test]
    fn right_border() {
        let mut world = World::new();
        world.insert(rltk::RandomNumberGenerator::new());
        let mut map = Map::new(&mut world);
        for idx in 0..(map.width * map.height - 1) {
            map.states[idx as usize] = 1
        }
        map.states[2 * map.width] = 2;
        assert_eq!(map.right(3 * map.width - 1), 2)
    }
}
