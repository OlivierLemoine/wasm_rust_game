use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn __log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($v: expr) => {
        self::__log($v)
    };
    ($($arg:tt)*) => {
        self::__log(format!($($arg)*))
    };
}

// pub use __console_log as console_log;
