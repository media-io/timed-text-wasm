#![feature(use_extern_macros)]

extern crate wasm_bindgen;
#[macro_use]
extern crate cfg_if;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert(&format!("Hello, Timed Text!"));
}
