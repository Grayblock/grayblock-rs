use log::Level;
use std::panic;
use wasm_bindgen::prelude::*;

pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();
    grayblock_frontend::new()
}
