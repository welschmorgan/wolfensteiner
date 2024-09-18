pub mod core;
pub mod js;

pub use core::*;
pub use js::*;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
fn init_wasm() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}
