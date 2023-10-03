use crate::world::Map;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Command {
    Nop,
    RotateLeft,
    RotateRight,
    Accelerate,
    Decelerate,
}

impl Command {
    pub fn exec(&self, pid: usize, map: &mut Map) {
        let p = match map.entities.players.get_mut(pid) {
            Some(p) => p,
            None => return,
        };

        match self {
            Command::Nop => (),
            Command::RotateLeft => {
                let rad =
                    cordic::atan2(p.body.rotation.y, p.body.rotation.x) - p.motion.rotation_speed;

                p.body.rotation.x = cordic::cos(rad);
                p.body.rotation.y = cordic::sin(rad);
            }
            Command::RotateRight => {
                let rad =
                    cordic::atan2(p.body.rotation.y, p.body.rotation.x) + p.motion.rotation_speed;

                p.body.rotation.x = cordic::cos(rad);
                p.body.rotation.y = cordic::sin(rad);
            }
            Command::Accelerate => {
                p.motion.speed += p.motion.acceleration;

                if p.motion.speed > p.motion.max_speed {
                    p.motion.speed = p.motion.max_speed;
                }
            }
            Command::Decelerate => {
                p.motion.speed -= p.motion.acceleration / 2;

                if p.motion.speed < -p.motion.max_speed / 2 {
                    p.motion.speed = -p.motion.max_speed / 2;
                }
            }
        }
    }
}
