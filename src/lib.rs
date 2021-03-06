use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

use legion::*;

mod components;
use components::{
    controller::Controller, static_graphic::StaticGraphic,
    transform::Tranform,
};

mod game_system;
use game_system::control::ControllerResource;

mod utils;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(raw_module = "../js/getResource.js")]
extern "C" {
    fn getResource(name: &str) -> web_sys::HtmlImageElement;
}

#[wasm_bindgen]
pub struct IntervalHandle {
    interval_id: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Drop for IntervalHandle {
    fn drop(&mut self) {
        let window = web_sys::window().unwrap();
        window.clear_interval_with_handle(self.interval_id);
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn main_js() -> Result<IntervalHandle, JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context: web_sys::CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let window = web_sys::window().unwrap();
    context.set_fill_style(&JsValue::from_str("black"));

    // test inti
    let game_world = Rc::new(RefCell::new(World::default()));
    {
        let mut w = game_world.borrow_mut();
        (*w).push((
            Tranform {
                velocity: vector2d::Vector2D::new(10., 10.),
                position: vector2d::Vector2D::new(0., 0.),
            },
            Controller {
                pressed: false,
                key: "",
            },
            StaticGraphic { name: "bullet" },
        ));
        (*w).push((
            Tranform {
                velocity: vector2d::Vector2D::new(20., 300.),
                position: vector2d::Vector2D::new(0., 0.),
            },
            StaticGraphic { name: "bullet" },
        ));
    }
    let game_schedule = Rc::new(RefCell::new(
        Schedule::builder()
            .add_system(game_system::control::control_update_system())
            .build(),
    ));
    let render_schedule = Rc::new(RefCell::new(
        Schedule::builder()
            .add_system(game_system::render::render_update_system())
            .build(),
    ));
    // keydown handler
    {
        let sched = game_schedule.clone();
        let w = Rc::clone(&game_world);
        let cb2 = Closure::wrap(Box::new(move |evt: web_sys::KeyboardEvent| {
            let mut realsch = sched.borrow_mut();
            let mut res: Resources = Resources::default();
            res.insert(ControllerResource {
                key: evt.key(),
                pressed: true,
            });
            (*realsch).execute(&mut w.borrow_mut(), &mut res);
        }) as Box<dyn FnMut(_)>);
        window.set_onkeydown(Some(cb2.as_ref().unchecked_ref()));
        cb2.forget();
    }

    // keyup handler
    {
        let sched = game_schedule.clone();
        let w = Rc::clone(&game_world);
        let cb2 = Closure::wrap(Box::new(move |evt: web_sys::KeyboardEvent| {
            let mut realsch = sched.borrow_mut();
            let mut res: Resources = Resources::default();
            res.insert(ControllerResource {
                key: evt.key(),
                pressed: false,
            });
            (*realsch).execute(&mut w.borrow_mut(), &mut res);
        }) as Box<dyn FnMut(_)>);
        window.set_onkeyup(Some(cb2.as_ref().unchecked_ref()));
        cb2.forget();
    }
    let cb = Closure::wrap(Box::new(move || {
        let mut w = game_world.borrow_mut();
        let mut r = Resources::default();
        let mut realsch = render_schedule.borrow_mut();
        (*realsch).execute(&mut w, &mut r);
    }) as Box<dyn FnMut()>);
    // game render loop
    let interval_id = window.set_interval_with_callback_and_timeout_and_arguments_0(
        cb.as_ref().unchecked_ref(),
        1000 / 60,
    )?;
    Ok(IntervalHandle {
        interval_id,
        _closure: cb,
    })
}
