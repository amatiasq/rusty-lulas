#![allow(dead_code)]

// mod utils;
mod dom;
mod simulation;

use dom::{get_size, render_frame, set_canvas_size};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn start() {
    let size = get_size();
    set_canvas_size(&size);
    simulation::start(&size);
}

#[wasm_bindgen]
pub fn update() {
    let cells = simulation::update();
    render_frame(&cells)
}
