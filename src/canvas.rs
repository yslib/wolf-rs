#![allow(dead_code)]
extern crate glm;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
use std::cell::{RefCell,RefMut};
use crate::palette::wolf_palette;

use super::math::{Vec2, Vec3};
use super::trait_def::{Primitive};


pub struct Texture2D<T>
where
    T: Primitive
{
    pub data: RefCell<Vec<T>>, // 4-bytes aligned
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
            data: RefCell::new(vec![T::default(); 0]),
            width: 0,
            height: 0,
            channel: 0,
        }
    }

}

impl<T> Texture2D<T> where T:Primitive{
    pub fn buffer(&self)->RefMut<Vec<T>> {
        self.data.borrow_mut()
    }
}


pub trait From2DData<U>{
    fn from_data(
        data: Vec<U>,
        width: u32,
        height: u32,
        channel: u8,
    )->Self;
}

impl<T, U> From2DData<U> for Texture2D<T>
where
    T: Primitive + From<U>
{
    fn from_data(
        data: Vec<U>,
        width: u32,
        height: u32,
        channel: u8,
    )->Self{
        Texture2D {
            data: RefCell::new(data.into_iter().map(|u|T::from(u)).collect()),
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
    palette:Vec<(u8,u8,u8,u8)>
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
            palette:wolf_palette()
        }
    }

    #[inline(always)]
    fn color_lut(&mut self, index:u8)->(u8,u8,u8,u8){
        self.palette[index as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let ind = (y as usize * self.size.0 + x as usize) * self.color_component;
        self.framebuffer[ind] = color.r;
        self.framebuffer[ind + 1] = color.g;
        self.framebuffer[ind + 2] = color.b;
        self.framebuffer[ind + 3] = color.a;
    }

    pub fn set_pixel_by_color_index(&mut self, x: u32, y: u32, index:u8) {
        let ind = (y as usize * self.size.0 + x as usize) * self.color_component;
        let color = self.color_lut(index);
        self.framebuffer[ind] = color.0;
        self.framebuffer[ind + 1] = color.1;
        self.framebuffer[ind + 2] = color.2;
        self.framebuffer[ind + 3] = color.3;
    }

    pub fn buffer_as_mut(&mut self) -> &mut [u8] {
        &mut self.framebuffer
    }

    pub fn clear(&mut self){
        self.framebuffer.fill(0xFF);
    }

    pub fn draw_ceil_and_floor(&mut self){
        let half = self.buffer_bytes/2;
        self.framebuffer[0..half].fill(64);
        self.framebuffer[half..].fill(128);
        self.framebuffer.iter_mut().skip(3).step_by(4).for_each(|v|*v=255);
    }

    pub fn set_wall(&mut self, col:usize, wall_color_index:&[u8]){
        let half_wall = wall_color_index.len() / 2;
        let half_canvas = self.size.1 / 2;
        let row = half_canvas - half_wall;
        for idx in row..(row+wall_color_index.len()){
            let comp = idx * self.size.0 + col;
            let index = comp * self.color_component;
            let color_index = wall_color_index[idx - row];
            let color = self.color_lut(color_index);
            self.framebuffer[index] = color.0;
            self.framebuffer[index + 1] = color.1;
            self.framebuffer[index + 2] = color.2;
            self.framebuffer[index + 3] = color.3;
        }
    }
}
