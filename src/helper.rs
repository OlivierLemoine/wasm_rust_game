use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn window() -> Result<web_sys::Window, JsValue> {
    web_sys::window().ok_or(JsValue::from(Error::new(&"No window")))
}

pub fn document() -> Result<web_sys::Document, JsValue> {
    window()?
        .document()
        .ok_or(JsValue::from(Error::new(&"No document")))
}

pub fn body() -> Result<web_sys::HtmlElement, JsValue> {
    document()?
        .body()
        .ok_or(JsValue::from(Error::new(&"No body")))
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) -> Result<(), JsValue> {
    window()?.request_animation_frame(f.as_ref().unchecked_ref())?;
    Ok(())
}
