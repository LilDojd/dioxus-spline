#![allow(non_snake_case)]

use dioxus::prelude::*;

use wasm_bindgen::prelude::JsCast;
mod runtime;
use runtime::Application;

#[derive(Props, PartialEq, Clone)]
pub struct SplineProps {
    #[props(into)]
    pub scene: String,
}

fn get_canvas_element() -> Option<web_sys::HtmlCanvasElement> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let canvas_element = document.get_element_by_id("canvas3d")?;
    let canvas = canvas_element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .ok()?;
    Some(canvas)
}

#[component]
pub fn Spline(props: SplineProps) -> Element {
    tracing::info!("Entered Spline");

    let mut canvas_element = use_signal(|| None::<web_sys::HtmlCanvasElement>);

    rsx!(canvas {
        id: "canvas3d",
        onmounted: move |_event| {
            if let Some(canvas_ele) = get_canvas_element() {
                let canvas_ref = canvas_element.get_or_insert(canvas_ele);
                let spline = Application::new(&canvas_ref, true);
                let props_clone = props.clone();
                spline.load(props_clone.scene);
            }
        }
    })
}
