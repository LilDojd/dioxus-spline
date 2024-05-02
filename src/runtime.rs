use core::fmt;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;

/// An enum matching @splinetool/runtime `SplineEventName tstype
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
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
    collision,
    rendered,
}

impl fmt::Display for SplineEventName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SplineEvent {
    pub target: Target,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Target {
    pub name: String,
    pub id: String,
}

impl From<JsValue> for SplineEvent {
    fn from(value: JsValue) -> Self {
        serde_wasm_bindgen::from_value(value).unwrap()
    }
}

#[wasm_bindgen(module = "/src/runtime.js")]
extern "C" {

    /// Binding to runtime.js Application
    #[derive(Clone)]
    pub type Application;

    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement, render_on_demand: bool) -> Application;

    #[wasm_bindgen(method)]
    pub async fn load(this: &Application, scene: String);

    #[wasm_bindgen(method)]
    pub fn addEventListener(
        this: &Application,
        event_name: &str,
        callback: &Closure<dyn FnMut(JsValue)>,
    );

    #[wasm_bindgen(method)]
    pub fn removeEventListener(this: &Application, event_name: &str);

    #[wasm_bindgen(method)]
    pub fn dispose(this: &Application);

    #[wasm_bindgen(method)]
    pub fn findObjectByName(this: &Application, name: String) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn findObjectById(this: &Application, id: String) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn getAllObjects(this: &Application) -> Vec<JsValue>;

    #[wasm_bindgen(method)]
    pub fn emitEvent(this: &Application, eventName: SplineEventName, nameOrUuid: String);

    #[wasm_bindgen(method)]
    pub fn emitEventReverse(this: &Application, eventName: SplineEventName, nameOrUuid: String);

    /// Experimental
    #[wasm_bindgen(method)]
    pub fn setZoom(this: &Application, zoomValue: f64);

    #[wasm_bindgen(method)]
    pub fn setBackgroundColor(this: &Application, color: String);

    /// Experimental
    #[wasm_bindgen(method)]
    pub fn setGlobalEvents(this: &Application, global: bool);

    #[wasm_bindgen(method)]
    pub fn setSize(this: &Application, width: f64, height: f64);

    /// Experimental
    #[wasm_bindgen(method, getter)]
    pub fn data(this: &Application) -> JsValue;

    /// Experimental
    #[wasm_bindgen(method, getter)]
    pub fn eventManager(this: &Application) -> JsValue;

    /// Experimental
    #[wasm_bindgen(method, getter)]
    pub fn controls(this: &Application) -> JsValue;

    /// Experimental
    #[wasm_bindgen(method, getter)]
    pub fn isStopped(this: &Application) -> bool;

    /// Experimental
    #[wasm_bindgen(method)]
    pub fn stop(this: &Application);

    /// Experimental
    #[wasm_bindgen(method)]
    pub fn play(this: &Application);

    /// Experimental
    #[wasm_bindgen(method)]
    pub fn requestRender(this: &Application);

    /// Experimental
    #[wasm_bindgen(method)]
    pub fn setVariables(this: &Application, variables: JsValue);

    /// Experimental
    #[wasm_bindgen(method)]
    pub fn setVariable(this: &Application, name: String, value: JsValue);

    /// Experimental
    #[wasm_bindgen(method)]
    pub fn getVariables(this: &Application) -> JsValue;

    /// Experimental
    #[wasm_bindgen(method)]
    pub fn getVariable(this: &Application, name: String) -> JsValue;
}
