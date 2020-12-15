use crate::components::{
    controller::Controller, static_graphic::StaticGraphic, transform::Tranform,
};
use crate::utils;
use legion::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "../js/getResource.js")]
extern "C" {
    fn getResource(name: &str) -> web_sys::HtmlImageElement;
}

#[system(for_each)]
pub fn render_update(ctrl: &mut Controller, transform: &mut Tranform, sprite: &StaticGraphic) {
    if ctrl.pressed {
        transform.position += transform.velocity
    };

    let context = utils::get_canvas();
    context.clear_rect(0., 0., 1280., 720.);
    context
        .draw_image_with_html_image_element(&getResource("forest00"), 0., 0.)
        .unwrap();
    context
        .draw_image_with_html_image_element(
            &getResource(sprite.name),
            transform.position.x as f64,
            transform.position.y as f64,
        )
        .unwrap();
}
