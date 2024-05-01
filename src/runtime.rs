use core::fmt;
use std::fmt::Debug;

use wasm_bindgen::prelude::*;
use web_sys::js_sys;
use web_sys::HtmlCanvasElement;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum SplineEventName {
    mouseDown,
    mouseUp,
    mouseHover,
    keyDown,
    keyUp,
    start,
    lookAt,
    follow,
    scroll,
}

impl fmt::Display for SplineEventName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[wasm_bindgen(module = "/src/runtime.js")]
extern "C" {

    pub type Application;

    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement, render_on_demand: bool) -> Application;

    #[wasm_bindgen(method)]
    pub fn load(this: &Application, scene: String);

    #[wasm_bindgen(method)]
    pub fn addEventListener(this: &Application, event_name: &str, callback: &js_sys::Function);

    #[wasm_bindgen(method)]
    pub fn removeEventListener(this: &Application, event_name: &str);

    #[wasm_bindgen(method)]
    pub fn dispose(this: &Application);

}
