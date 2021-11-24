use std::ops::{Add, Div, Mul, Sub};

pub trait Primitive: Send + Copy + Sized + PartialOrd + PartialEq + Default + Sub + Add + Mul + Div{}

impl Primitive for f32 {}

impl Primitive for u8 {}

impl Primitive for i32 {}

impl Primitive for u32 {}

impl Primitive for i64 {}

impl Primitive for u64 {}

pub trait FromRef<T> {
    fn from_ref(t: &T) -> Self;
}

impl FromRef<u8> for f32 {
    fn from_ref(t: &u8) -> f32 {
        *t as f32
    }
}
