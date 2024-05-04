#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::runtime::{SplineApplication, SplineEvent, SplineEventName};
use wasm_bindgen::{JsCast, JsValue};

#[derive(Props, PartialEq, Clone)]
pub struct SplineProps {
    #[props(into)]
    pub scene: String,
    pub on_load: Option<EventHandler<SplineApplication>>,
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

fn _event_factory(
    props: &SplineProps,
) -> Vec<(SplineEventName, Option<EventHandler<SplineEvent>>)> {
    vec![
        (SplineEventName::mouseDown, props.on_mouse_down),
        (SplineEventName::mouseUp, props.on_mouse_up),
        (SplineEventName::mouseHover, props.on_mouse_hover),
        (SplineEventName::keyDown, props.on_key_down),
        (SplineEventName::keyUp, props.on_key_up),
        (SplineEventName::start, props.on_start),
        (SplineEventName::lookAt, props.on_look_at),
        (SplineEventName::follow, props.on_follow),
        (SplineEventName::scroll, props.on_wheel),
    ]
}
/// You can pass just the scene `String` to this component for simple renders. Alternatively,
/// you can also attach event handlers to attributes defined in [SplineProps].
/// If you would like to store SPEObject and modify it with events, you can use `use_signal` the following way:
///
/// # Example
///
/// *In the component*
/// ```
/// let mut splineobject = use_signal(|| None::<SPEObject>);
///
/// rsx! {
///     Spline {
///         scene: String::from("my-awesome-scene.splinecode")
///         on_load: move |event: Application| {
///             let obj = event.findObjectByName(String::from("Cube"));
///             // Store SPEObject
///             splineobject.write().insert(SPEObject::new(obj));
///         }
///     }
/// }
///
/// ```
///
/// Then you can listen to `SplineEvents` or rescale/move objects with other events:
///
/// # Example
///
/// ```
/// button {
///   onclick: move |_| {
///        let mut spe_object = splineobject.unwrap();
///        let mut new_scale = spe_object.scale();
///        new_scale += SPEVector3::new(0.5, 0.5, 0.5);
///        spe_object.set_scale(&new_scale);
///    },
///   "Make helix chonky!"
/// }
/// ```
/// **Notice the difference between on_click and onclick!**

#[component]
pub fn Spline(props: SplineProps) -> Element {
    let mut app = use_signal(|| None::<SplineApplication>);
    let scene = use_signal(|| props.scene.clone());
    let props_cloned = props.clone();

    // Load scene and attach events
    let _ = use_resource(move || {
        let events = _event_factory(&props_cloned);

        async move {
            app.unwrap().load(scene()).await;

            for (event_name, handler) in events {
                if let Some(handler) = handler {
                    let cb = move |event: JsValue| {
                        let event: SplineEvent = event.into();
                        handler.call(event);
                    };

                    tracing::info!("Adding event listener for {:?}", event_name);
                    app.unwrap()
                        .add_event_listener(event_name.to_string().as_str(), cb);
                }
            }
            if let Some(on_load) = props_cloned.on_load {
                on_load.call(app.unwrap())
            }
        }
    });

    rsx! {
        canvas {
            onmounted: move |event: Event<MountedData>| {
                let canvas_ref = get_raw_canvas_element(&event.data);
                let render_on_demand = props.render_on_demand.unwrap_or(true);
                app.set(Some(SplineApplication::new(canvas_ref, render_on_demand)));
            }
        }
        // TODO: is_loading

        // match &*load_scene.read_unchecked() {
        //     Some(_) => rsx! {"Loaded"},
        //     None => rsx! {"Loading"}
        // }

    }
}
