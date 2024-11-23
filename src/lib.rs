// This is a library, it doesn't use all its own functions
#![allow(dead_code)]
// And stuff that wasm doesn't want / care about / allow
#![allow(clippy::new_without_default)]
#![allow(clippy::must_use_candidate)]

extern crate wasm_bindgen;
extern crate web_sys;

#[macro_use]
mod engine;

mod piece;
mod board;

#[allow(clippy::module_name_repetitions)]
mod game;
