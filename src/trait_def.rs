use std::{ops::{Add, Div, Mul, Sub}, process::Output};

pub trait Primitive:
    Send + Copy + Sized + PartialOrd + PartialEq + Default + Sub<Output=Self> + Add<Output=Self> + Mul<Output=Self> + Div<Output=Self>
{
}

impl Primitive for f32 {}

impl Primitive for u8 {}

impl Primitive for i32 {}

impl Primitive for u32 {}

impl Primitive for i64 {}

impl Primitive for u64 {}