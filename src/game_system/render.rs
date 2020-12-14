use crate::components::{controller::Controller, transform::Tranform};
use legion::*;

#[system(for_each)]
pub fn render_update(ctrl: &mut Controller, transform: &mut Tranform) {
    if ctrl.pressed {
        transform.position += transform.velocity
    }
}
