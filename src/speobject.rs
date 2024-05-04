use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq)]
pub struct SPEObject {
    inner: JsValue,
    pub position: SPEVector3,
    pub scale: SPEVector3,
}

impl SPEObject {
    pub fn new(obj: JsValue) -> SPEObject {
        let position = Reflect::get(&obj, &"position".into())
            .ok()
            .map(SPEVector3::from_jsvalue)
            .unwrap_or_else(|| SPEVector3::new(0.0, 0.0, 0.0));
        let scale = Reflect::get(&obj, &"scale".into())
            .ok()
            .map(SPEVector3::from_jsvalue)
            .unwrap_or_else(|| SPEVector3::new(1.0, 1.0, 1.0));
        SPEObject {
            inner: obj,
            position,
            scale,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SPEVector3 {
    inner: JsValue,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl SPEVector3 {
    fn new(x: f64, y: f64, z: f64) -> SPEVector3 {
        let obj = Object::new();
        SPEVector3 {
            inner: obj.into(),
            x,
            y,
            z,
        }
    }

    fn from_jsvalue(value: JsValue) -> SPEVector3 {
        let x = Reflect::get(&value, &"x".into())
            .expect("Did not receive x")
            .as_f64()
            .expect("x is not a f64");
        let y = Reflect::get(&value, &"y".into())
            .expect("Did not receive y")
            .as_f64()
            .expect("y is not a f64");
        let z = Reflect::get(&value, &"z".into())
            .expect("Did not receive z")
            .as_f64()
            .expect("z is not a f64");
        SPEVector3 {
            inner: value,
            x,
            y,
            z,
        }
    }

    fn update_js_fields(&self) {
        Reflect::set(&self.inner, &"x".into(), &self.x.into()).unwrap();
        Reflect::set(&self.inner, &"y".into(), &self.y.into()).unwrap();
        Reflect::set(&self.inner, &"z".into(), &self.z.into()).unwrap();
    }
}

impl Drop for SPEVector3 {
    fn drop(&mut self) {
        self.update_js_fields();
    }
}
