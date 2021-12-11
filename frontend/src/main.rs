use log::Level;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

use grayblock_frontend::{home, layout};

pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let component = Component::from(layout::view(home::view()));
    let view = component.build().unwrap();

    view.run()
}
