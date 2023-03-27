mod canvas;
mod canvas_context;

use self::{canvas::Canvas, canvas_context::CanvasContext};
use crate::simulation::{cell::Cell, vector::Vector};
use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Window};

pub struct Renderer {
    window: web_sys::Window,
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

fn get_window() -> Window {
    web_sys::window().expect("Can't find window")
}

fn get_document(window: Window) -> Document {
    window.document().expect("Can't find document")
}

pub fn get_canvas(document: Document) -> Canvas {
    let canvas = document
        .get_element_by_id("canvas")
        .expect("Can't find canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Can't cast canvas");

    Canvas(canvas)
}

pub fn get_size() -> Vector {
    let window = get_window();

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

    Vector {
        x: width,
        y: height,
    }
}

pub fn set_canvas_size(size: &Vector) {
    let window = get_window();
    let canvas = get_canvas(get_document(window));

    canvas.set_width(size.x as u32);
    canvas.set_height(size.y as u32);
}

pub fn render_frame(cells: &Vec<Cell>) {
    let canvas = get_canvas(get_document(get_window()));
    let context = canvas.clear();

    for cell in cells {
        draw_cell(&context, cell);
    }
}

fn draw_cell(context: &CanvasContext, cell: &Cell) {
    context.begin_path();
    context.arc(
        cell.position.x,
        cell.position.y,
        5.0,
        0.0,
        f64::consts::PI * 2.0,
    );
    context.stroke();
    context.close_path();
}

fn similey(context: &CanvasContext) {
    context.begin_path();

    // Draw the outer circle.
    context.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0);

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI);

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context.arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0);

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context.arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0);

    context.stroke()
}
