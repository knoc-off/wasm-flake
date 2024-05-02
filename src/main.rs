use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
use noise::{NoiseFn, Perlin};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

pub fn main() -> Result<(), JsValue> {
    let window = window().ok_or("should have a window in this context")?;
    let document = window.document().ok_or("window should have a document")?;

    let canvas = document
        .get_element_by_id("perlinCanvas")
        .ok_or("should have #perlinCanvas on the page")?
        .dyn_into::<HtmlCanvasElement>()?;

    // Set canvas size to match the window's inner dimensions
    canvas.set_width(window.inner_width()?.as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height()?.as_f64().unwrap() as u32);

    let context = canvas
        .get_context("2d")?
        .ok_or("Failed to get 2D context")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    let perlin = Perlin::new(12);
    draw_perlin_noise(&perlin, &context, canvas.width() as usize, canvas.height() as usize);
    Ok(())
}

fn draw_perlin_noise(perlin: &Perlin, context: &CanvasRenderingContext2d, width: usize, height: usize) {
    for x in 0..width {
        for y in 0..height {
            let noise_val = perlin.get([x as f64 * 0.1, y as f64 * 0.1]);
            let color_value = ((noise_val + 1.0) / 2.0 * 255.0) as u8;
            let color = format!("rgb({0},{0},{0})", color_value);
            context.set_fill_style(&JsValue::from_str(&color));
            context.fill_rect(x as f64, y as f64, 1.0, 1.0);
        }
    }
}

