extern crate glm;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
use super::math::{Vec2, Vec3};
use super::trait_def::{FromRef, Primitive};


pub struct Texture2D<T>
where
    T: Primitive,
{
    pub data: Vec<T>, // 4-bytes aligned
    pub width: u32,
    pub height: u32,
    pub channel: u8,
}


impl<T> Default for Texture2D<T>
where
    T: Primitive,
{
    fn default() -> Self {
        Texture2D {
            data: vec![T::default(); 0],
            width: 0,
            height: 0,
            channel: 0,
        }
    }
}

fn my_convert2<T, U>(v:&[U])->Vec<T>
where
    T:FromRef<U>
{
    v.iter().map(T::from_ref).collect()
}

pub trait From2DData<U>{
    fn from_data(
        data: &[U],
        width: u32,
        height: u32,
        channel: u8,
    )->Self;
}

impl<T, U> From2DData<U> for Texture2D<T>
where
    T: Primitive + FromRef<U>,
{
    fn from_data(
        data: &[U],
        width: u32,
        height: u32,
        channel: u8,
    )->Self{
        Texture2D {
            data: my_convert2(&data),
            width: width,
            height: height,
            channel: channel,
        }
    }
}

pub struct Canvas {
    size: (usize, usize),
    framebuffer: Vec<u8>,
    color_component: usize,
    pixel_count: usize,
    buffer_bytes: usize,
}

impl Canvas {
    pub fn new(res: (usize, usize)) -> Self {
        let comp = 4usize;
        Canvas {
            size: res,
            framebuffer: vec![0u8; res.0 * res.1 * comp],
            color_component: comp,
            pixel_count: res.0 * res.1,
            buffer_bytes: res.0 * res.1 * comp,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let ind = y as usize * self.size.0 + x as usize;
        self.framebuffer[ind] = color.r;
        self.framebuffer[ind + 1] = color.g;
        self.framebuffer[ind + 2] = color.b;
        self.framebuffer[ind + 3] = color.a;
    }

    pub fn buffer_as_mut(&mut self) -> &mut [u8] {
        &mut self.framebuffer
    }
}
