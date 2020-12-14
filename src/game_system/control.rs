use crate::components::{controller::Controller, transform::Tranform};
use legion::*;

pub struct ControllerResource {
    pub key: String,
    pub pressed: bool,
}

#[system(for_each)]
pub fn control_update(
    ctrl: &mut Controller,
    trans: &mut Tranform,
    #[resource] res: &ControllerResource,
) {
    ctrl.key = Box::leak(res.key.clone().into_boxed_str());
    ctrl.pressed = res.pressed;
    match ctrl.key {
        "w" => {
            if trans.position.y - trans.velocity.y > 0f32 {
                trans.velocity = vector2d::Vector2D::new(0., -32.)
            }
        }
        "s" => {
            if trans.position.y + trans.velocity.y < 720f32 {
                trans.velocity = vector2d::Vector2D::new(0., 32.)
            }
        }
        "a" => trans.velocity = vector2d::Vector2D::new(-32., 0.),
        "d" => trans.velocity = vector2d::Vector2D::new(32., 0.),
        _ => (),
    };
}
