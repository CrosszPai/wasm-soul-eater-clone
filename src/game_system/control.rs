use crate::components::{controller::Controller};
use legion::*;

pub struct ControllerResource {
    pub key: String,
    pub pressed: bool,
}

#[system(for_each)]
pub fn control_update(ctrl: &mut Controller, #[resource] res: &ControllerResource) {
    ctrl.key = Box::leak(res.key.clone().into_boxed_str());
    ctrl.pressed = res.pressed;
}
