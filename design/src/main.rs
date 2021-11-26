#![allow(unused_braces)]
use log::Level;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

pub mod connect_button;

fn view() -> ViewBuilder<Dom> {
    builder! {
        <div>
            "Grayblock Component Design System"
            {connect_button::new()}
        </div>
    }
}

fn app() -> Component<Dom> {
    let app_view = view();
    Component::from(app_view)
}

pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let view = app().build().unwrap();

    view.run().unwrap();

    Ok(())
}
