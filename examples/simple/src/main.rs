#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_spline::{SPEObject, SPEVector3, Spline, SplineApplication, SplineEvent};
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

pub fn App() -> Element {
    let mut splineobject = use_signal(|| None::<SPEObject>);
    let mut number_events_received = use_signal(|| 0);
    let mut target = use_signal(|| String::from("None"));
    rsx! {
        div { width: "800px",
            Spline {
                scene: String::from("https://prod.spline.design/PWOr9wT1pcAkbAA7/scene.splinecode"),
                on_load: move |event: SplineApplication| {
                    let obj = event.find_object_by_name(String::from("Helix 2"));
                    let _ = splineobject.write().insert(SPEObject::new(obj));
                    tracing::info!("object: {splineobject:?}");
                },
                on_mouse_down: move |event: SplineEvent| {
                    tracing::info!("Recieved {event:?}");
                    number_events_received += 1;
                    let recent_target = event.target.name;
                    target.set(recent_target);
                },
                on_mouse_hover: move |event: SplineEvent| {
                    tracing::info!("Mouse hover event {event:?}");
                    number_events_received += 1;
                    let recent_target = event.target.name;
                    target.set(recent_target);
                }
            }
        }
        button {
            onclick: move |_| {
                let mut spe_object = splineobject.unwrap();
                let mut new_scale = spe_object.scale();
                new_scale += SPEVector3::new(0.5, 0.5, 0.5);
                spe_object.set_scale(&new_scale);
            },
            "Make helix chonky!"
        }
        h1 { "Received {number_events_received} Spline events. (Click on blue helix/hover over text)" }
        p { "Most recent target: {target} "}
    }
}
