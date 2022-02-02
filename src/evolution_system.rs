use super::Map;
use specs::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Evolution;

impl<'a> System<'a> for Evolution {
    type SystemData = WriteExpect<'a, Map>;

    fn run(&mut self, data: Self::SystemData) {
        let mut map = data;
        let next = map.clone();
        for idx in 0..(map.width * map.height - 1) {
            if next.states[idx as usize] == map.range - 1 {
                if (next.up(idx as usize) == 0)
                    || (next.down(idx as usize) == 0)
                    || (next.left(idx as usize) == 0)
                    || (next.right(idx as usize) == 0)
                {
                    map.states[idx as usize] = 0;
                }
            } else {
                if (next.up(idx as usize) - next.states[idx as usize] == 1)
                    || (next.down(idx as usize) - next.states[idx as usize] == 1)
                    || (next.left(idx as usize) - next.states[idx as usize] == 1)
                    || (next.right(idx as usize) - next.states[idx as usize] == 1)
                {
                    map.states[idx as usize] += 1;
                }
            }
        }
    }
}
