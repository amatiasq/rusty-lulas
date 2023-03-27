use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use super::canvas_context::CanvasContext;

pub struct Canvas(pub HtmlCanvasElement);

impl Canvas {
    pub fn get_context_2d(&self) -> CanvasContext {
        let context = self
            .0
            .get_context("2d")
            .expect("Can't get context")
            .expect("POLLLA")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("Can't cast context");

        CanvasContext(context)
    }

    pub fn clear(&self) -> CanvasContext {
        let context = self.get_context_2d();
        let width = self.width();
        let height = self.height();
        context.clear_rect(0.0, 0.0, width as f64, height as f64);
        context
    }

    //

    pub fn width(&self) -> u32 {
        self.0.width()
    }

    pub fn set_width(&self, value: u32) {
        self.0.set_width(value);
    }

    pub fn height(&self) -> u32 {
        self.0.height()
    }

    pub fn set_height(&self, value: u32) {
        self.0.set_height(value);
    }

    // pub fn get_context(&self, context_id: &str) -> Option<::js_sys::Object> {
    //     self.0.get_context(context_id)
    // }

    // pub fn get_context_with_context_options(
    //     &self,
    //     context_id: &str,
    //     context_options: &::wasm_bindgen::JsValue,
    // ) -> Option<::js_sys::Object> {
    //     self
    //         .0
    //         .get_context_with_context_options(context_id, context_options)
    // }

    // pub fn to_blob(&self, callback: &::js_sys::Function) {
    //     self.0.to_blob(callback);
    // }

    // pub fn to_blob_with_type(&self, callback: &::js_sys::Function, type_: &str) {
    //     self.0.to_blob_with_type(callback, type_);
    // }

    // pub fn to_blob_with_type_and_encoder_options(
    //     &self,
    //     callback: &::js_sys::Function,
    //     type_: &str,
    //     encoder_options: &::wasm_bindgen::JsValue,
    // ) {
    //     elf
    //         .0
    //         .to_blob_with_type_and_encoder_options(callback, type_, encoder_options);
    // }

    // pub fn to_data_url(&self) -> String {
    //     self.0.to_data_url()
    // }

    // pub fn to_data_url_with_type(&self, type_: &str) -> String {
    //     self.0.to_data_url_with_type(type_)
    // }

    // pub fn to_data_url_with_type_and_encoder_options(
    //     &self,
    //     type_: &str,
    //     encoder_options: &::wasm_bindgen::JsValue,
    // ) -> String {
    //     self
    //         .0
    //         .to_data_url_with_type_and_encoder_options(type_, encoder_options)
    // }

    // pub fn transfer_control_to_offscreen(&self) -> OffscreenCanvas {
    //     self.0.transfer_control_to_offscreen()
    // }
}
