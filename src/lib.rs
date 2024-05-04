//! This crate provides a simple [dioxus] component for rendering [Spline] scenes in web.
//! It also exposes interfaces to `SPEObject` and Spline `Application` from @splinetool/runtime.
//! Currently, it is highly experimental and a lot of functionality is untested, but rendering
//! Scenes and simple callbacks should work just fine!
//!
//! 🌈 Spline is a friendly 3d collaborative design tool for the web.
//!
//! [dioxus]: https://dioxuslabs.com/
//! [Spline]: https://spline.design/
//!
//! # Usage
//!
//! First, add this to your Cdargo.toml
//!
//! ```toml
//! [dependencies]
//! dioxus-spline = "0.1.0"
//! ```
//!
//! Next add the `Spline` component to your dioxus `App`:
//!
//! ```
//! fn App() -> Element {
//!     rsx! {
//!         Spline { scene: String::from("https://prod.spline.design/PWOr9wT1pcAkbAA7/scene.splinecode") }
//!     }
//! }
//! ```
//!
//! You should see your scene now!

/// The runtime with `Application` and `SplineEvent`
pub mod runtime;
/// The wrapper around JS `SPEObject`
pub mod speobject;
/// The `Spline` component itself
pub mod spline;

pub use runtime::{SplineApplication, SplineEvent, SplineEventName};
pub use speobject::{SPEObject, SPEVector3};
pub use spline::Spline;
