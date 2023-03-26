// mod utils;
mod render;
mod simulation;

use render::Renderer;
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
    let renderer = Renderer::new();
    let size = &renderer.get_size();
    let mut cells = simulation::start(size);

    // loop {
    simulation::update(&mut cells);
    renderer.frame(&cells)
    // }
}
