use crate::simulation::{cell::Cell, vector::Vector};
use std::f64;
use wasm_bindgen::prelude::*;

pub struct Renderer {
    window: web_sys::Window,
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

impl Renderer {
    pub fn new() -> Renderer {
        let window = web_sys::window().expect("Can't find window");
        let document = window.document().expect("Can't find document");
        let canvas = document
            .get_element_by_id("canvas")
            .expect("Can't find canvas");

        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .expect("Can't cast canvas");

        let context = canvas
            .get_context("2d")
            .expect("Can't get context")
            .expect("Can't get context 2")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("Can't cast context");

        Renderer {
            window,
            context,
            canvas,
        }
    }

    pub fn get_size(&self) -> Vector {
        let window = &self.window;

        let width = window
            .inner_width()
            .expect("Can't get width")
            .as_f64()
            .expect("Can't parse width");

        let height = window
            .inner_height()
            .expect("Can't get height")
            .as_f64()
            .expect("Can't parse height");

        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);

        Vector {
            x: width,
            y: height,
        }
    }

    pub fn frame(&self, cells: &Vec<Cell>) {
        let context = &self.context;

        for cell in cells.iter() {
            draw_cell(&context, cell);
        }
    }
}

fn draw_cell(context: &web_sys::CanvasRenderingContext2d, cell: &Cell) {
    context.begin_path();
    context
        .arc(
            cell.position.x,
            cell.position.y,
            5.0,
            0.0,
            f64::consts::PI * 2.0,
        )
        .expect("Can't draw arc");
    context.stroke();
    context.close_path();
}

fn similey(renderer: &Renderer) {
    let context = &renderer.context;

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .expect("Can't draw arc");

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context
        .arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI)
        .expect("Can't draw arc 2");

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .expect("Can't draw arc 3");

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .expect("Can't draw arc 4");

    context.stroke()
}
