use std::ops::{Add, AddAssign};

use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq)]
pub struct SPEObject {
    inner: JsValue,
}

/// The SPEObject interface

impl SPEObject {
    /// # Example
    ///
    /// ```
    /// let speobj = event.find_object_by_name(String::from("Helix 2"));
    /// ```
    pub fn new(obj: JsValue) -> SPEObject {
        SPEObject { inner: obj }
    }

    pub fn position(&self) -> SPEVector3 {
        SPEVector3::from_jsvalue(
            Reflect::get(&self.inner, &"position".into()).expect("Did not receive position"),
        )
    }

    pub fn scale(&self) -> SPEVector3 {
        SPEVector3::from_jsvalue(
            Reflect::get(&self.inner, &"scale".into()).expect("Did not receive scale"),
        )
    }

    pub fn set_position(&mut self, position: &SPEVector3) {
        let mut obj_position = self.position();
        obj_position.set_x(position.x());
        obj_position.set_y(position.y());
        obj_position.set_z(position.z());
    }

    pub fn set_scale(&mut self, scale: &SPEVector3) {
        let mut obj_scale = self.scale();
        obj_scale.set_x(scale.x());
        obj_scale.set_y(scale.y());
        obj_scale.set_z(scale.z());
    }
}

/// A wrapper around JsValue(x, y, z)
/// You can create a vector, set its x/y/z and set SPEObject scale or position with this vector
#[derive(Debug, Clone, PartialEq)]
pub struct SPEVector3 {
    inner: JsValue,
}

impl SPEVector3 {
    pub fn new(x: f64, y: f64, z: f64) -> SPEVector3 {
        let obj = Object::new();
        Reflect::set(&obj, &"x".into(), &x.into()).unwrap();
        Reflect::set(&obj, &"y".into(), &y.into()).unwrap();
        Reflect::set(&obj, &"z".into(), &z.into()).unwrap();
        SPEVector3 { inner: obj.into() }
    }

    pub fn from_jsvalue(value: JsValue) -> SPEVector3 {
        SPEVector3 { inner: value }
    }

    pub fn x(&self) -> f64 {
        Reflect::get(&self.inner, &"x".into())
            .expect("Did not receive x")
            .as_f64()
            .expect("x is not a f64")
    }

    pub fn set_x(&mut self, value: f64) {
        Reflect::set(&self.inner, &"x".into(), &value.into()).unwrap();
    }

    pub fn y(&self) -> f64 {
        Reflect::get(&self.inner, &"y".into())
            .expect("Did not receive y")
            .as_f64()
            .expect("y is not a f64")
    }

    pub fn set_y(&mut self, value: f64) {
        Reflect::set(&self.inner, &"y".into(), &value.into()).unwrap();
    }

    pub fn z(&self) -> f64 {
        Reflect::get(&self.inner, &"z".into())
            .expect("Did not receive z")
            .as_f64()
            .expect("z is not a f64")
    }

    pub fn set_z(&mut self, value: f64) {
        Reflect::set(&self.inner, &"z".into(), &value.into()).unwrap();
    }
}

impl Add for SPEVector3 {
    type Output = SPEVector3;

    fn add(self, other: SPEVector3) -> SPEVector3 {
        SPEVector3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl AddAssign for SPEVector3 {
    fn add_assign(&mut self, other: SPEVector3) {
        self.set_x(self.x() + other.x());
        self.set_y(self.y() + other.y());
        self.set_z(self.z() + other.z());
    }
}
