use super::{State, World, MAPCOUNT, MAPHEIGHT, MAPWIDTH, NUMCOLORS};
use rltk::{RandomNumberGenerator, Rltk, RGB};
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

        for idx in 0..(MAPCOUNT) {
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
            self.states[idx + 1 - self.width]
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

pub fn draw(ecs: &mut State, ctx: &mut Rltk) {
    let map = ecs.ecs.fetch::<Map>();
    let mut y = 0;
    let mut x = 0;

    for (idx, tile) in map.states.iter().enumerate() {
        let glyph = rltk::to_cp437('.');
        let mut fg = RGB::from_f32(0., 0., 0.);
        let mut bg = RGB::from_f32(0., 0., 0.);
        match tile {
            0 => {
                fg = RGB::from_f32(0., 0., 0.);
                bg = RGB::from_f32(0., 0., 0.);
            }
            1 => {
                fg = RGB::from_f32(255., 0., 0.);
                bg = RGB::from_f32(255., 0., 0.);
            }
            2 => {
                fg = RGB::from_f32(0., 255., 0.);
                bg = RGB::from_f32(0., 255., 0.);
            }
            3 => {
                fg = RGB::from_f32(0., 0., 255.);
                bg = RGB::from_f32(0., 0., 255.);
            }
            4 => {
                fg = RGB::from_f32(128., 128., 0.);
                bg = RGB::from_f32(128., 128., 0.);
            }
            5 => {
                fg = RGB::from_f32(128., 0., 128.);
                bg = RGB::from_f32(128., 0., 128.);
            }
            6 => {
                fg = RGB::from_f32(0., 128., 128.);
                bg = RGB::from_f32(0., 128., 128.);
            }
            7 => {
                fg = RGB::from_f32(128., 128., 128.);
                bg = RGB::from_f32(128., 128., 128.);
            }
            8 => {
                fg = RGB::from_f32(255., 255., 0.);
                bg = RGB::from_f32(255., 255., 0.);
            }
            9 => {
                fg = RGB::from_f32(255., 0., 255.);
                bg = RGB::from_f32(255., 0., 255.);
            }
            _ => {
                bg = RGB::from_f32(0., 255., 255.);
                fg = RGB::from_f32(0., 255., 255.);
            }
        }
        ctx.set(x, y, fg, bg, glyph);

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
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

    #[test]
    fn map_size() {
        let mut world = World::new();
        world.insert(rltk::RandomNumberGenerator::new());
        let mut map = Map::new(&mut world);
        assert_eq!(map.width * map.height, map.states.len());
    }
}
