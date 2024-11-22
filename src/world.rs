use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct World
{
	width: f64,
	height: f64,
}
impl World
{
	pub fn new(width: f64, height: f64) -> Self
	{		
		World {
			width,
			height,
		}
	}	
	
	pub fn update(&mut self, deltatime: f64)
	{
		//
	}
	
	pub fn get_width  (&self) -> f64 { self.width }
	pub fn get_height (&self) -> f64 { self.height }

	/// Inform the World module that the mouse has been moved
	pub fn mouse_moved(&mut self, x: i32, y:i32)
	{
		//
	}
	pub fn mouse_pressed(&mut self, x: i32, y: i32)
	{
		//
	}

	pub fn draw(&mut self, context: &CanvasRenderingContext2d)
	{
		//
	}
}
