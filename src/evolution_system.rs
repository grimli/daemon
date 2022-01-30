use super::Map;
use specs::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Evolution;

impl<'a> System<'a> for Evolution {
    type SystemData = WriteExpect<'a, Map>;

    fn run(&mut self, data: Self::SystemData) {
        let (mut map) = data;
        self.evolve(&mut map);
        map.draw();
    }
}

impl Evolution {
    fn evolve(self, map: &mut Map) {
        let mut next = map.clone();
        for idx in 0..(map.width * map.height - 1) {
            let numcolors = map.range - 1;
            match map.states[idx as usize] {
                numcolors => {
                    if (map.up(idx as usize) == 0)
                        || (map.down(idx as usize) == 0)
                        || (map.left(idx as usize) == 0)
                        || (map.right(idx as usize) == 0)
                    {
                        next.states[idx as usize] = 0;
                    }
                }
                _ => {
                    if (map.up(idx as usize) - map.states[idx as usize] == 1)
                        || (map.down(idx as usize) - map.states[idx as usize] == 1)
                        || (map.left(idx as usize) - map.states[idx as usize] == 1)
                        || (map.right(idx as usize) - map.states[idx as usize] == 1)
                    {
                        next.states[idx as usize] += 1;
                    }
                }
            }
        }
    }
}
