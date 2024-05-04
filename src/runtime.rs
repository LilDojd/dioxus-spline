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

    #[derive(Clone)]
    #[wasm_bindgen]
    type Application;

    #[wasm_bindgen(constructor)]
    fn new_(canvas: &HtmlCanvasElement, render_on_demand: bool) -> Application;

    #[wasm_bindgen(method, js_name = load)]
    async fn load_(this: &Application, scene: String);

    #[wasm_bindgen(method, js_name = addEventListener)]
    fn add_event_listener_(
        this: &Application,
        event_name: &str,
        callback: &Closure<dyn FnMut(JsValue)>,
    );

    #[wasm_bindgen(method, js_name = removeEventListener)]
    fn remove_event_listener_(this: &Application, event_name: &str);

    #[wasm_bindgen(method, js_name = dispose)]
    fn dispose_(this: &Application);

    #[wasm_bindgen(method, js_name = findObjectByName)]
    fn find_object_by_name_(this: &Application, name: String) -> JsValue;

    #[wasm_bindgen(method, js_name = findObjectById)]
    fn find_object_by_id_(this: &Application, id: String) -> JsValue;

    #[wasm_bindgen(method, js_name = getAllObjects)]
    fn get_all_objects_(this: &Application) -> Vec<JsValue>;

    #[wasm_bindgen(method, js_name = emitEvent)]
    fn emit_event_(this: &Application, eventName: SplineEventName, nameOrUuid: String);

    #[wasm_bindgen(method, js_name = emitEventReverse)]
    fn emit_event_reverse_(this: &Application, eventName: SplineEventName, nameOrUuid: String);

    #[wasm_bindgen(method, js_name = setZoom)]
    fn set_zoom_(this: &Application, zoomValue: f64);

    #[wasm_bindgen(method, js_name = setBackgroundColor)]
    fn set_background_color_(this: &Application, color: String);

    /// Experimental
    #[wasm_bindgen(method, js_name = setGlobalEvents)]
    fn set_global_events_(this: &Application, global: bool);

    #[wasm_bindgen(method, js_name = setSize)]
    fn set_size_(this: &Application, width: f64, height: f64);

    /// Experimental
    #[wasm_bindgen(method, getter, js_name = data)]
    fn data_(this: &Application) -> JsValue;

    /// Experimental
    #[wasm_bindgen(method, getter, js_name = eventManager)]
    fn event_manager_(this: &Application) -> JsValue;

    /// Experimental
    #[wasm_bindgen(method, getter, js_name = controls)]
    fn controls_(this: &Application) -> JsValue;

    /// Experimental
    #[wasm_bindgen(method, getter, js_name = isStopped)]
    fn is_stopped_(this: &Application) -> bool;

    /// Experimental
    #[wasm_bindgen(method, js_name = stop)]
    fn stop_(this: &Application);

    /// Experimental
    #[wasm_bindgen(method, js_name = play)]
    fn play_(this: &Application);

    /// Experimental
    #[wasm_bindgen(method, js_name = requestRender)]
    fn request_render_(this: &Application);

    /// Experimental
    #[wasm_bindgen(method, js_name = setVariables)]
    fn set_variables_(this: &Application, variables: JsValue);

    /// Experimental
    #[wasm_bindgen(method, js_name = setVariable)]
    fn set_variable_(this: &Application, name: String, value: JsValue);

    /// Experimental
    #[wasm_bindgen(method, js_name = getVariables)]
    fn get_variables_(this: &Application) -> JsValue;

    /// Experimental
    #[wasm_bindgen(method, js_name = getVariable)]
    fn get_variable_(this: &Application, name: String) -> JsValue;
}

#[derive(Clone)]
pub struct SplineApplication {
    app: Application,
}

impl SplineApplication {
    pub fn new(canvas: &HtmlCanvasElement, render_on_demand: bool) -> Self {
        let app = Application::new_(canvas, render_on_demand);
        SplineApplication { app }
    }

    pub async fn load(&self, scene: String) {
        self.app.load_(scene).await;
    }

    pub fn add_event_listener<F>(&self, event_name: &str, callback: F)
    where
        F: FnMut(JsValue) + 'static,
    {
        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut(JsValue)>);
        self.app.add_event_listener_(event_name, &closure);
        closure.forget();
    }

    pub fn remove_event_listener(&self, event_name: &str) {
        self.app.remove_event_listener_(event_name);
    }

    pub fn dispose(&self) {
        self.app.dispose_();
    }

    pub fn find_object_by_name(&self, name: String) -> JsValue {
        self.app.find_object_by_name_(name)
    }

    pub fn find_object_by_id(&self, id: String) -> JsValue {
        self.app.find_object_by_id_(id)
    }

    pub fn get_all_objects(&self) -> Vec<JsValue> {
        self.app.get_all_objects_()
    }

    #[deprecated(note = "Experimental")]
    pub fn emit_event(&self, event_name: SplineEventName, name_or_uuid: String) {
        self.app.emit_event_(event_name, name_or_uuid);
    }

    #[deprecated(note = "Experimental")]
    pub fn emit_event_reverse(&self, event_name: SplineEventName, name_or_uuid: String) {
        self.app.emit_event_reverse_(event_name, name_or_uuid);
    }

    #[deprecated(note = "Experimental")]
    pub fn set_zoom(&self, zoom_value: f64) {
        self.app.set_zoom_(zoom_value);
    }

    pub fn set_background_color(&self, color: String) {
        self.app.set_background_color_(color);
    }

    #[deprecated(note = "Experimental")]
    pub fn set_global_events(&self, global: bool) {
        self.app.set_global_events_(global);
    }

    pub fn set_size(&self, width: f64, height: f64) {
        self.app.set_size_(width, height);
    }

    #[deprecated(note = "Experimental")]
    pub fn data(&self) -> JsValue {
        self.app.data_()
    }

    #[deprecated(note = "Experimental")]
    pub fn event_manager(&self) -> JsValue {
        self.app.event_manager_()
    }

    #[deprecated(note = "Experimental")]
    pub fn controls(&self) -> JsValue {
        self.app.controls_()
    }

    #[deprecated(note = "Experimental")]
    pub fn is_stopped(&self) -> bool {
        self.app.is_stopped_()
    }

    #[deprecated(note = "Experimental")]
    pub fn stop(&self) {
        self.app.stop_();
    }

    #[deprecated(note = "Experimental")]
    pub fn play(&self) {
        self.app.play_();
    }

    #[deprecated(note = "Experimental")]
    pub fn request_render(&self) {
        self.app.request_render_();
    }

    #[deprecated(note = "Experimental")]
    pub fn set_variables(&self, variables: JsValue) {
        self.app.set_variables_(variables);
    }

    #[deprecated(note = "Experimental")]
    pub fn set_variable(&self, name: String, value: JsValue) {
        self.app.set_variable_(name, value);
    }

    #[deprecated(note = "Experimental")]
    pub fn get_variables(&self) -> JsValue {
        self.app.get_variables_()
    }

    #[deprecated(note = "Experimental")]
    pub fn get_variable(&self, name: String) -> JsValue {
        self.app.get_variable_(name)
    }

    pub fn get_app(&self) -> &Application {
        &self.app
    }
}
