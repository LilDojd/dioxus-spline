#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_spline::Spline;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

pub fn App() -> Element {
    rsx! {
        Spline {
            scene: String::from("https://prod.spline.design/PWOr9wT1pcAkbAA7/scene.splinecode"),
            on_mouse_down: |event| {
                tracing::info!("Mouse down event {event:#?}");
            },
            on_mouse_hover: |_| {
                tracing::info!("Mouse hover event")
            }
        }
    }
}
