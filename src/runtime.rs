use wasm_bindgen::prelude::*;
use web_sys::js_sys;
use web_sys::HtmlCanvasElement;
#[wasm_bindgen(module = "/src/runtime.js")]
extern "C" {
    pub type Application;

    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement, render_on_demand: bool) -> Application;

    #[wasm_bindgen(method)]
    pub fn load(this: &Application, scene: String);

    #[wasm_bindgen(method)]
    pub fn add_event_listener(this: &Application, event_name: &str, callback: &js_sys::Function);

    #[wasm_bindgen(method)]
    pub fn remove_event_listener(this: &Application, event_name: &str);

    #[wasm_bindgen(method)]
    pub fn dispose(this: &Application);

}
