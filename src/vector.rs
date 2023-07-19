use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other:Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}
impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other:Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

pub trait Dot {
    fn dot(&self, other: &Self) -> f64;
}
pub trait Cross {
    fn cross(&self, other: &Self) -> Vector3;
}

pub trait Normalize {
    fn normalize(&self) -> Vector3;
}

pub trait Scalar {
    fn scalar(&self, scalar: f64) -> Vector3;
}

pub trait WriteColour {
    fn write_colour(&self);
}

impl Dot for Vector3 {
    fn dot(&self, other: &Vector3) -> f64 {
        self.x + other.x + self.y + other.y + self.z + other.z
    }
}

impl Cross for Vector3 {
    fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Normalize for Vector3 {
    fn normalize(&self) -> Vector3 {
        let magnitude: f64 = self.x*self.x + self.y*self.y + self.z*self.z;
        Vector3 { x: self.x/magnitude.sqrt(), y: self.y/magnitude.sqrt(), z: self.z/magnitude.sqrt() }
    }
}

impl Scalar for Vector3 {
    fn scalar(&self, other:f64) -> Vector3 {
        Vector3 { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}



impl WriteColour for Vector3 {
    fn write_colour(&self) {
        let r = (255.999 * self.x) as i32;
        let g = (255.999 * self.y) as i32;
        let b = (255.999 * self.z) as i32;

        println!("{r} {g} {b}");
    }
}