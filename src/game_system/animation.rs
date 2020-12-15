use crate::components::{sprite_graphic::SpriteGraphic, transform::Tranform};
use crate::utils;
use legion::*;

#[system(for_each)]
pub fn animation_update(sprite: &SpriteGraphic, transform: &Tranform) {
    let context = utils::get_canvas();
    let src = utils::getResource(sprite.name);
    if sprite.mirror {
        context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &src,
                0.,
                0.,
                32.,
                32.,
                transform.position.x,
                transform.position.y,
                32.,
                32.,
            )
            .unwrap()
    }
}
