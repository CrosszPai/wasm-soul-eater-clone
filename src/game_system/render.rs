use crate::components::{controller::Controller, position::Position, velocity::Velocity};
use legion::*;

#[system(for_each)]
pub fn render_update(ctrl: &mut Controller, pos: &mut Position, vel: &Velocity) {
    if ctrl.pressed {
        match ctrl.key {
            "w" => {
                if pos.x - vel.dy > 0f32 {
                    pos.set_y(pos.y - vel.dy as f32);
                }
            }
            "s" => {
                if pos.y + vel.dy < 720f32 {
                    pos.set_y(pos.y + vel.dy as f32)
                }
            }
            "a" => pos.set_x(pos.x - vel.dx),
            "d" => pos.set_x(pos.x + vel.dx),
            _ => (),
        };
    }
}
