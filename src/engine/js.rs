use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern
{
	pub fn alert(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	pub fn log(s: &str);
}

// Stolen from https://rustwasm.github.io/wasm-bindgen/examples/console-log.html
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (js::log(&format_args!($($t)*).to_string()))
}
