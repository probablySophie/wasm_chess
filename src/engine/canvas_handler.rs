#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]

use wasm_bindgen::prelude::*;
use crate::engine::js;

pub fn setup(width: f64, height: f64)
{

	let window: web_sys::Window = web_sys::window().expect("Unable to load Window");
	let document = window.document().expect("Unable to load Document from Window");
	let device_pixel_ratio = window.device_pixel_ratio();
	
	console_log!("Seting up canvas!");
	
	let canvas = document.get_element_by_id("canvas").expect("Failed to find canvas");

	let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

	
	let context = canvas.get_context("2d")
			.expect("Canvas unwrap 1 failed")
			.expect("Canvas unwrap 2 failed")
			.dyn_into::<web_sys::CanvasRenderingContext2d>()
			.expect("Failed to dyn_into the canvas context");

	let width:u32  = (width  * device_pixel_ratio) as u32;
	let height:u32 = (height * device_pixel_ratio) as u32;

	console_log!("{}, {}", width, height);
	
	// Fix how the canvas will draw for high DPI screens
	canvas.set_width (width);
	canvas.set_height(height);

	let _ = canvas.set_attribute("width" , &width .to_string());
	let _ = canvas.set_attribute("height", &height.to_string());
	
	context.set_font("16px Arial");
	context.set_image_smoothing_enabled(false);
}
