use std::{
    ops::{Add, Div, Mul, Sub},
};

use rayon::iter::Copied;

pub trait Primitive:
    Send
    + Copy
    + Sized
    + PartialOrd
    + PartialEq
    + Default
    + Sub<Output = Self>
    + Add<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    fn invsqrt(self) -> Self;
}

impl Primitive for f32 {
    fn invsqrt(self) -> Self {
        1f32/self.sqrt()
    }
}

impl Primitive for u8 {
    fn invsqrt(self) -> Self {
        (1.0 / (self as f64).sqrt()) as u8
    }
}

impl Primitive for i32 {
    fn invsqrt(self) -> Self {
        (1.0 / (self as f64).sqrt()) as i32
    }
}

impl Primitive for u32 {
    fn invsqrt(self) -> Self {
        (1.0 / (self as f64).sqrt()) as u32
    }
}

impl Primitive for i64 {
    fn invsqrt(self) -> Self {
        (1.0 / (self as f64).sqrt()) as i64
    }
}

impl Primitive for u64 {
    fn invsqrt(self) -> Self {
        (1.0 / (self as f64).sqrt()) as u64
    }
}


// impl Primitive for f32 {
//     fn invsqrt(self) -> f32 {
//         1f32/self.sqrt()
//     }
// }

// impl Primitive for u8 {
//     fn invsqrt(self) -> f32 {
//         1.0 / (self as f32).sqrt()
//     }
// }

// impl Primitive for i32 {
//     fn invsqrt(self) -> f32 {
//         1.0 / (self as f32).sqrt()
//     }
// }

// impl Primitive for u32 {
//     fn invsqrt(self) -> f32 {
//         1.0 / (self as f32).sqrt()
//     }
// }

// impl Primitive for i64 {
//     fn invsqrt(self) -> f32 {
//         1.0 / (self as f32).sqrt()
//     }
// }

// impl Primitive for u64 {
//     fn invsqrt(self) -> f32 {
//         1.0 / (self as f32).sqrt()
//     }
// }

pub trait NumVec<T>
where
    T: Primitive,
{
    fn sum(&self) -> T;
    fn prod(&self) -> T;
}
