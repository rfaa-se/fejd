use crate::world::Map;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Command {
    Nop,
    Left,
    Right,
}

impl Command {
    pub fn exec(&self, pid: usize, map: &mut Map) {
        match self {
            Command::Nop => (),
            Command::Left => {
                let p = match map.entities.players.get_mut(pid) {
                    Some(p) => p,
                    None => return,
                };

                let rad = cordic::atan2(p.body.rotation.y, p.body.rotation.x) - p.rotation_speed;

                p.body.rotation.x = cordic::cos(rad);
                p.body.rotation.y = cordic::sin(rad);
            }
            Command::Right => {
                let p = match map.entities.players.get_mut(pid) {
                    Some(p) => p,
                    None => return,
                };

                // TODO: fix constants instead

                let rad = cordic::atan2(p.body.rotation.y, p.body.rotation.x) + p.rotation_speed;

                p.body.rotation.x = cordic::cos(rad);
                p.body.rotation.y = cordic::sin(rad);
            }
        }
    }
}
