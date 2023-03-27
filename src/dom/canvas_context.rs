use web_sys::CanvasRenderingContext2d;

use super::canvas::Canvas;

pub struct CanvasContext(pub CanvasRenderingContext2d);

impl CanvasContext {
    pub fn canvas(&self) -> Option<Canvas> {
        self.0.canvas().map(Canvas)
    }

    pub fn global_alpha(&self) -> f64 {
        self.0.global_alpha()
    }

    pub fn set_global_alpha(&self, value: f64) {
        self.0.set_global_alpha(value);
    }

    pub fn global_composite_operation(&self) -> String {
        self.0
            .global_composite_operation()
            .expect("global_composite_operation failed")
    }

    pub fn set_global_composite_operation(&self, value: &str) {
        self.0
            .set_global_composite_operation(value)
            .expect("set_global_composite_operation failed");
    }

    pub fn stroke_style(&self) -> ::wasm_bindgen::JsValue {
        self.0.stroke_style()
    }

    pub fn set_stroke_style(&self, value: &::wasm_bindgen::JsValue) {
        self.0.set_stroke_style(value);
    }

    pub fn fill_style(&self) -> ::wasm_bindgen::JsValue {
        self.0.fill_style()
    }

    pub fn set_fill_style(&self, value: &::wasm_bindgen::JsValue) {
        self.0.set_fill_style(value);
    }

    pub fn filter(&self) -> String {
        self.0.filter()
    }

    pub fn set_filter(&self, value: &str) {
        self.0.set_filter(value);
    }

    pub fn image_smoothing_enabled(&self) -> bool {
        self.0.image_smoothing_enabled()
    }

    pub fn set_image_smoothing_enabled(&self, value: bool) {
        self.0.set_image_smoothing_enabled(value);
    }

    pub fn line_width(&self) -> f64 {
        self.0.line_width()
    }

    pub fn set_line_width(&self, value: f64) {
        self.0.set_line_width(value);
    }

    pub fn line_cap(&self) -> String {
        self.0.line_cap()
    }

    pub fn set_line_cap(&self, value: &str) {
        self.0.set_line_cap(value);
    }

    pub fn line_join(&self) -> String {
        self.0.line_join()
    }

    pub fn set_line_join(&self, value: &str) {
        self.0.set_line_join(value);
    }

    pub fn miter_limit(&self) -> f64 {
        self.0.miter_limit()
    }

    pub fn set_miter_limit(&self, value: f64) {
        self.0.set_miter_limit(value);
    }

    pub fn line_dash_offset(&self) -> f64 {
        self.0.line_dash_offset()
    }

    pub fn set_line_dash_offset(&self, value: f64) {
        self.0.set_line_dash_offset(value);
    }

    pub fn shadow_offset_x(&self) -> f64 {
        self.0.shadow_offset_x()
    }

    pub fn set_shadow_offset_x(&self, value: f64) {
        self.0.set_shadow_offset_x(value);
    }

    pub fn shadow_offset_y(&self) -> f64 {
        self.0.shadow_offset_y()
    }

    pub fn set_shadow_offset_y(&self, value: f64) {
        self.0.set_shadow_offset_y(value);
    }

    pub fn shadow_blur(&self) -> f64 {
        self.0.shadow_blur()
    }

    pub fn set_shadow_blur(&self, value: f64) {
        self.0.set_shadow_blur(value);
    }

    pub fn shadow_color(&self) -> String {
        self.0.shadow_color()
    }

    pub fn set_shadow_color(&self, value: &str) {
        self.0.set_shadow_color(value);
    }

    pub fn font(&self) -> String {
        self.0.font()
    }

    pub fn set_font(&self, value: &str) {
        self.0.set_font(value);
    }

    pub fn text_align(&self) -> String {
        self.0.text_align()
    }

    pub fn set_text_align(&self, value: &str) {
        self.0.set_text_align(value);
    }

    pub fn text_baseline(&self) -> String {
        self.0.text_baseline()
    }

    pub fn set_text_baseline(&self, value: &str) {
        self.0.set_text_baseline(value);
    }

    // pub fn draw_window(&self, window: &Window, x: f64, y: f64, w: f64, h: f64, bg_color: &str) {
    //     self.0.draw_window(window, x, y, w, h, bg_color);
    // }

    // pub fn draw_window_with_flags(&self, window: &Window, x: f64, y: f64, w: f64, h: f64, bg_color: &str, flags: u32) {
    //     self.0.draw_window_with_flags(window, x, y, w, h, bg_color, flags);
    // }

    // pub fn draw_image_with_html_image_element(&self, image: &HtmlImageElement, dx: f64, dy: f64) {
    //     self.0.draw_image_with_html_image_element(image, dx, dy);
    // }

    // pub fn draw_image_with_svg_image_element(&self, image: &SvgImageElement, dx: f64, dy: f64) {
    //     self.0.draw_image_with_svg_image_element(image, dx, dy);
    // }

    // pub fn draw_image_with_html_canvas_element(&self, image: &HtmlCanvasElement, dx: f64, dy: f64) {
    //     self.0.draw_image_with_html_canvas_element(image, dx, dy);
    // }

    // pub fn draw_image_with_html_video_element(&self, image: &HtmlVideoElement, dx: f64, dy: f64) {
    //     self.0.draw_image_with_html_video_element(image, dx, dy);
    // }

    // pub fn draw_image_with_image_bitmap(&self, image: &ImageBitmap, dx: f64, dy: f64) {
    //     self.0.draw_image_with_image_bitmap(image, dx, dy);
    // }

    // pub fn draw_image_with_offscreen_canvas(&self, image: &OffscreenCanvas, dx: f64, dy: f64) {
    //     self.0.draw_image_with_offscreen_canvas(image, dx, dy);
    // }

    // pub fn draw_image_with_video_frame(&self, image: &VideoFrame, dx: f64, dy: f64) {
    //     self.0.draw_image_with_video_frame(image, dx, dy);
    // }

    // pub fn draw_image_with_html_image_element_and_dw_and_dh(&self, image: &HtmlImageElement, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_html_image_element_and_dw_and_dh(image, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_svg_image_element_and_dw_and_dh(&self, image: &SvgImageElement, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_svg_image_element_and_dw_and_dh(image, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_html_canvas_element_and_dw_and_dh(&self, image: &HtmlCanvasElement, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_html_canvas_element_and_dw_and_dh(image, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_html_video_element_and_dw_and_dh(&self, image: &HtmlVideoElement, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_html_video_element_and_dw_and_dh(image, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_image_bitmap_and_dw_and_dh(&self, image: &ImageBitmap, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_image_bitmap_and_dw_and_dh(image, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_offscreen_canvas_and_dw_and_dh(&self, image: &OffscreenCanvas, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_offscreen_canvas_and_dw_and_dh(image, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_video_frame_and_dw_and_dh(&self, image: &VideoFrame, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_video_frame_and_dw_and_dh(image, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &HtmlImageElement, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(image, sx, sy, sw, sh, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &SvgImageElement, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(image, sx, sy, sw, sh, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &HtmlCanvasElement, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(image, sx, sy, sw, sh, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &HtmlVideoElement, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(image, sx, sy, sw, sh, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &ImageBitmap, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(image, sx, sy, sw, sh, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_offscreen_canvas_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &OffscreenCanvas, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_offscreen_canvas_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(image, sx, sy, sw, sh, dx, dy, dw, dh);
    // }

    // pub fn draw_image_with_video_frame_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &VideoFrame, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
    //     self.0.draw_image_with_video_frame_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(image, sx, sy, sw, sh, dx, dy, dw, dh);
    // }

    pub fn begin_path(&self) {
        self.0.begin_path();
    }

    pub fn clip(&self) {
        self.0.clip();
    }

    // pub fn clip_with_canvas_winding_rule(&self, winding: CanvasWindingRule) {
    //     self.0.clip_with_canvas_winding_rule(winding);
    // }

    // pub fn clip_with_path_2d(&self, path: &Path2d) {
    //     self.0.clip_with_path_2d(path);
    // }

    // pub fn clip_with_path_2d_and_winding(&self, path: &Path2d, winding: CanvasWindingRule) {
    //     self.0.clip_with_path_2d_and_winding(path, winding);
    // }

    pub fn fill(&self) {
        self.0.fill();
    }

    // pub fn fill_with_canvas_winding_rule(&self, winding: CanvasWindingRule) {
    //     self.0.fill_with_canvas_winding_rule(winding);
    // }

    // pub fn fill_with_path_2d(&self, path: &Path2d) {
    //     self.0.fill_with_path_2d(path);
    // }

    // pub fn fill_with_path_2d_and_winding(&self, path: &Path2d, winding: CanvasWindingRule) {
    //     self.0.fill_with_path_2d_and_winding(path, winding);
    // }

    // pub fn is_point_in_path_with_f64(&self, x: f64, y: f64) -> bool {
    //     self.0.is_point_in_path_with_f64(x, y)
    // }

    // pub fn is_point_in_path_with_f64_and_canvas_winding_rule(&self, x: f64, y: f64, winding: CanvasWindingRule) -> bool {
    //     self.0.is_point_in_path_with_f64_and_canvas_winding_rule(x, y, winding)
    // }

    // pub fn is_point_in_path_with_path_2d_and_f64(&self, path: &Path2d, x: f64, y: f64) -> bool {
    //     self.0.is_point_in_path_with_path_2d_and_f64(path, x, y)
    // }

    // pub fn is_point_in_path_with_path_2d_and_f64_and_winding(&self, path: &Path2d, x: f64, y: f64, winding: CanvasWindingRule) -> bool {
    //     self.0.is_point_in_path_with_path_2d_and_f64_and_winding(path, x, y, winding)
    // }

    // pub fn is_point_in_stroke_with_x_and_y(&self, x: f64, y: f64) -> bool {
    //     self.0.is_point_in_stroke_with_x_and_y(x, y)
    // }

    // pub fn is_point_in_stroke_with_path_and_x_and_y(&self, path: &Path2d, x: f64, y: f64) -> bool {
    //     self.0.is_point_in_stroke_with_path_and_x_and_y(path, x, y)
    // }

    pub fn stroke(&self) {
        self.0.stroke();
    }

    // pub fn stroke_with_path(&self, path: &Path2d) {
    //     self.0.stroke_with_path(path);
    // }

    #[cfg(feature = "CanvasGradient")]
    pub fn create_linear_gradient(&self, x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient {
        self.0.create_linear_gradient(x0, y0, x1, y1)
    }

    // pub fn create_pattern_with_html_image_element(&self, image: &HtmlImageElement, repetition: &str) -> Option<CanvasPattern> {
    //     self.0.create_pattern_with_html_image_element(image, repetition) Option<CanvasPattern>;
    // }

    // pub fn create_pattern_with_svg_image_element(&self, image: &SvgImageElement, repetition: &str) -> Option<CanvasPattern> {
    //     self.0.create_pattern_with_svg_image_element(image, repetition) Option<CanvasPattern>;
    // }

    // pub fn create_pattern_with_html_canvas_element(&self, image: &HtmlCanvasElement, repetition: &str) -> Option<CanvasPattern> {
    //     self.0.create_pattern_with_html_canvas_element(image, repetition) Option<CanvasPattern>;
    // }

    // pub fn create_pattern_with_html_video_element(&self, image: &HtmlVideoElement, repetition: &str) -> Option<CanvasPattern> {
    //     self.0.create_pattern_with_html_video_element(image, repetition) Option<CanvasPattern>;
    // }

    // pub fn create_pattern_with_image_bitmap(&self, image: &ImageBitmap, repetition: &str) -> Option<CanvasPattern> {
    //     self.0.create_pattern_with_image_bitmap(image, repetition) Option<CanvasPattern>;
    // }

    // pub fn create_pattern_with_offscreen_canvas(&self, image: &OffscreenCanvas, repetition: &str) -> Option<CanvasPattern> {
    //     self.0.create_pattern_with_offscreen_canvas(image, repetition) Option<CanvasPattern>;
    // }

    // pub fn create_pattern_with_video_frame(&self, image: &VideoFrame, repetition: &str) -> Option<CanvasPattern> {
    //     self.0.create_pattern_with_video_frame(image, repetition) Option<CanvasPattern>;
    // }

    #[cfg(feature = "CanvasGradient")]
    pub fn create_radial_gradient(
        &self,
        x0: f64,
        y0: f64,
        r0: f64,
        x1: f64,
        y1: f64,
        r1: f64,
    ) -> CanvasGradient {
        self.0.create_radial_gradient(x0, y0, r0, x1, y1, r1)
    }

    pub fn add_hit_region(&self) {
        self.0.add_hit_region().expect("add_hit_region failed")
    }

    // pub fn add_hit_region_with_options(&self, options: &HitRegionOptions) {
    //     self.0.add_hit_region_with_options(options);
    // }

    pub fn clear_hit_regions(&self) {
        self.0.clear_hit_regions();
    }

    pub fn remove_hit_region(&self, id: &str) {
        self.0.remove_hit_region(id);
    }

    // pub fn create_image_data_with_sw_and_sh(&self, sw: f64, sh: f64) -> ImageData {
    //     self.0.create_image_data_with_sw_and_sh(sw, sh)
    // }

    // pub fn create_image_data_with_imagedata(&self, imagedata: &ImageData) -> ImageData {
    //     self.0.create_image_data_with_imagedata(imagedata)
    // }

    // pub fn get_image_data(&self, sx: f64, sy: f64, sw: f64, sh: f64) -> ImageData {
    //     self.0.get_image_data(sx, sy, sw, sh)
    // }

    // pub fn put_image_data(&self, imagedata: &ImageData, dx: f64, dy: f64) {
    //     self.0.put_image_data(imagedata, dx, dy);
    // }

    // pub fn put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(&self, imagedata: &ImageData, dx: f64, dy: f64, dirty_x: f64, dirty_y: f64, dirty_width: f64, dirty_height: f64) {
    //     self.0.put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(imagedata, dx, dy, dirty_x, dirty_y, dirty_width, dirty_height);
    // }

    // pub fn get_line_dash(&self) -> ::js_sys::Array {
    //     self.0.get_line_dash()
    // }

    pub fn set_line_dash(&self, segments: &::wasm_bindgen::JsValue) {
        self.0
            .set_line_dash(segments)
            .expect("set_line_dash failed");
    }

    pub fn arc(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        self.0
            .arc(x, y, radius, start_angle, end_angle)
            .expect("arc failed");
    }

    pub fn arc_with_anticlockwise(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) {
        self.0
            .arc_with_anticlockwise(x, y, radius, start_angle, end_angle, anticlockwise)
            .expect("arc_with_anticlockwise failed");
    }

    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        self.0
            .arc_to(x1, y1, x2, y2, radius)
            .expect("arc_to failed");
    }

    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.0.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    pub fn close_path(&self) {
        self.0.close_path();
    }

    pub fn ellipse(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) {
        self.0
            .ellipse(x, y, radius_x, radius_y, rotation, start_angle, end_angle)
            .expect("ellipse failed");
    }

    pub fn ellipse_with_anticlockwise(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) {
        self.0
            .ellipse_with_anticlockwise(
                x,
                y,
                radius_x,
                radius_y,
                rotation,
                start_angle,
                end_angle,
                anticlockwise,
            )
            .expect("ellipse_with_anticlockwise failed")
    }

    pub fn line_to(&self, x: f64, y: f64) {
        self.0.line_to(x, y);
    }

    pub fn move_to(&self, x: f64, y: f64) {
        self.0.move_to(x, y);
    }

    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.0.quadratic_curve_to(cpx, cpy, x, y);
    }

    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) {
        self.0.rect(x, y, w, h);
    }

    pub fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        self.0.clear_rect(x, y, w, h);
    }

    pub fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        self.0.fill_rect(x, y, w, h);
    }

    pub fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        self.0.stroke_rect(x, y, w, h);
    }

    pub fn restore(&self) {
        self.0.restore();
    }

    pub fn save(&self) {
        self.0.save();
    }

    pub fn fill_text(&self, text: &str, x: f64, y: f64) {
        self.0.fill_text(text, x, y).expect("fill_text failed");
    }

    pub fn fill_text_with_max_width(&self, text: &str, x: f64, y: f64, max_width: f64) {
        self.0
            .fill_text_with_max_width(text, x, y, max_width)
            .expect("fill_text_with_max_width failed");
    }

    // pub fn measure_text(&self, text: &str) -> TextMetrics {
    //     self.0.measure_text(text) TextMetrics;
    // }

    pub fn stroke_text(&self, text: &str, x: f64, y: f64) {
        self.0.stroke_text(text, x, y).expect("stroke_text failed");
    }

    pub fn stroke_text_with_max_width(&self, text: &str, x: f64, y: f64, max_width: f64) {
        self.0
            .stroke_text_with_max_width(text, x, y, max_width)
            .expect("stroke_text_with_max_width failed");
    }

    // pub fn get_transform(&self) -> DomMatrix {
    //     self.0.get_transform()
    // }

    pub fn reset_transform(&self) {
        self.0.reset_transform().expect("reset_transform failed");
    }

    pub fn rotate(&self, angle: f64) {
        self.0.rotate(angle).expect("rotate failed");
    }

    pub fn scale(&self, x: f64, y: f64) {
        self.0.scale(x, y).expect("scale failed");
    }

    pub fn set_transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.0
            .set_transform(a, b, c, d, e, f)
            .expect("set_transform failed");
    }

    pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.0
            .transform(a, b, c, d, e, f)
            .expect("transform failed");
    }

    pub fn translate(&self, x: f64, y: f64) {
        self.0.translate(x, y).expect("translate failed");
    }

    // pub fn draw_custom_focus_ring(&self, element: &Element) -> bool {
    //     self.0.draw_custom_focus_ring(element)
    // }

    // pub fn draw_focus_if_needed(&self, element: &Element) {
    //     self.0.draw_focus_if_needed(element)
    // }
}
