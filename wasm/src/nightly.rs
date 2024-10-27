use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
fn start() {
    console_log::init_with_level(log::Level::Trace).expect("could not initialize log");
}
