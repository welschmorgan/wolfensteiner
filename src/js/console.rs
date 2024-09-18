use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace = console)]
extern "C" {
    pub fn log(s: String);
}

#[macro_export]
macro_rules! console_log {
    ($fmt:expr) => {
        $crate::log(format!("{}", $fmt));
    };
    ($fmt:expr, $($args:expr),*) => {
        $crate::log(format!($fmt, $( $args ),*));
    }
}
