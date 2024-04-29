#![allow(non_snake_case)]

use dioxus::prelude::*;

mod runtime;
use runtime::{Application, SplineEvent, SplineEventName};
use wasm_bindgen::{closure::Closure, JsCast};

#[derive(Props, PartialEq, Clone)]
pub struct SplineProps {
    #[props(into)]
    pub scene: String,
    pub on_load: Option<EventHandler<Application>>,
    pub on_mouse_down: Option<EventHandler<SplineEvent>>,
    pub on_mouse_up: Option<EventHandler<SplineEvent>>,
    pub on_mouse_hover: Option<EventHandler<SplineEvent>>,
    pub on_key_down: Option<EventHandler<SplineEvent>>,
    pub on_key_up: Option<EventHandler<SplineEvent>>,
    pub on_start: Option<EventHandler<SplineEvent>>,
    pub on_look_at: Option<EventHandler<SplineEvent>>,
    pub on_follow: Option<EventHandler<SplineEvent>>,
    pub on_wheel: Option<EventHandler<SplineEvent>>,
    pub render_on_demand: Option<bool>,
}

/// Utility to get the raw canvas element from its mounted data.
fn get_raw_canvas_element(mounted: &MountedData) -> &web_sys::HtmlCanvasElement {
    mounted
        .downcast::<web_sys::Element>()
        .unwrap()
        .dyn_ref::<web_sys::HtmlCanvasElement>()
        .unwrap()
}

#[component]
pub fn Spline(props: SplineProps) -> Element {
    let mut is_loading = use_signal(|| true);

    rsx!(canvas {
        onmounted: move |event| {
            let canvas_ref = get_raw_canvas_element(&event);
            let render_on_demand = props.render_on_demand.unwrap_or(true);
            let spline = Application::new(canvas_ref, render_on_demand);
            let events = vec![
                (SplineEventName::mouseDown, props.on_mouse_down),
                (SplineEventName::mouseUp, props.on_mouse_up),
                (SplineEventName::mouseHover, props.on_mouse_hover),
                (SplineEventName::keyDown, props.on_key_down),
                (SplineEventName::keyUp, props.on_key_up),
                (SplineEventName::start, props.on_start),
                (SplineEventName::lookAt, props.on_look_at),
                (SplineEventName::follow, props.on_follow),
                (SplineEventName::scroll, props.on_wheel),
            ];
            for (event_name, handler) in events {
                if let Some(handler) = handler {
                    let cb = Closure::wrap(
                        Box::new(move |event| handler.call(event)) as Box<dyn FnMut(_)>
                    );
                    tracing::info!("Adding event listener for {:?}", event_name);
                    spline.addEventListener(
                        event_name.to_string().as_str(),
                        cb.as_ref().unchecked_ref(),
                    );
                    cb.forget();
                }
            }
            let props_clone = props.clone();
            spline.load(props_clone.scene);
            is_loading.set(false);
            if let Some(on_load) = props.on_load {
                on_load.call(spline)
            }
        }
    })
}
