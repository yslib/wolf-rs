#![allow(dead_code)]
#![allow(unused_variables)]
use std::f32::consts::FRAC_PI_2;

use glm::radians;

use super::trait_def::Primitive;

#[derive(Copy, Clone)]
pub struct Vec3<T>
where
    T: Primitive,
{
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T>
where
    T: Primitive,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3::<T> { x, y, z }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec2<T>
where
    T: Primitive,
{
    x: T,
    y: T,
}

impl<T> Vec2<T>
where
    T: Primitive,
{
    pub fn new(x: T, y: T) -> Self {
        Vec2::<T> { x, y }
    }
}

#[derive(Copy, Clone)]
pub struct Bound2<T: Primitive> {
    min: Vec2<T>,
    max: Vec2<T>,
}

impl<T: Primitive> Bound2<T> {
    fn new(min: Vec2<T>, max: Vec2<T>) -> Self {
        Bound2::<T> { min: min, max: max }
    }
}

impl<T: Primitive> Default for Bound2<T> {
    fn default() -> Self {
        Bound2::<T> {
            min: Vec2::<T>::new(T::default(), T::default()),
            max: Vec2::<T>::new(T::default(), T::default()),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Grid2 {
    pub bound: Bound2<i32>,
}

// Bresenham
pub struct DDAIterator<'a> {
    proj_x: f32,
    proj_step: Vec2<f32>,
    proj: Vec2<f32>,
    radians: f32,
    cell_index: Vec2<i32>,
    bound: &'a Bound2<i32>,
}

impl<'a> DDAIterator<'a> {
    fn new(pos: Vec2<f32>, radians: f32, bound: &'a Bound2<i32>) -> Self {
        let proj_step = Vec2::<f32> {
            x: 1_f32,
            y: (FRAC_PI_2 - radians).tan(),
        };
        let cell_index = Vec2::<i32> { x: pos.x as i32, y: pos.y as i32 };
		let init_tria = (pos.x.ceil() - pos.x,pos.y.ceil()-pos.y);
        let proj = Vec2::<f32>::new(init_tria.0, init_tria.1 * (FRAC_PI_2 - radians).tan());
        DDAIterator {
            proj_step: proj_step,
            cell_index: cell_index,
            proj_x: 0f32,
            proj: proj,
            bound: bound,
            radians: radians,
        }
    }
}

impl<'a> Iterator for DDAIterator<'a> {
    type Item = (Vec2<i32>, Vec2<f32>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.proj.x < self.proj.y {
            self.proj_x = self.proj.x;
            self.proj.x += self.proj_step.x;
            if self.proj_step.x < f32::EPSILON {
                self.cell_index.x -= 1;
            } else {
                self.cell_index.x += 1;
            }
        } else if self.proj.x > self.proj.y {
            self.proj_x = self.proj.y;
            self.proj.y += self.proj_step.y;
            if self.proj_step.y < f32::EPSILON {
                self.cell_index.y -= 1;
            } else {
                self.cell_index.y += 1;
            }
        } else {
            self.proj_x = self.proj.y;
            self.proj.y += self.proj_step.y;
            self.proj.x += self.proj_step.x;
            if self.proj_step.x < f32::EPSILON {
                self.cell_index.x -= 1;
            } else {
                self.cell_index.x += 1;
            }
            if self.proj_step.y < f32::EPSILON {
                self.cell_index.y -= 1;
            } else {
                self.cell_index.y += 1;
            }
        }
        if (self.cell_index.x >= self.bound.min.x && self.cell_index.x < self.bound.max.x)
            && (self.cell_index.y >= self.bound.min.y && self.cell_index.y < self.bound.max.y)
        {
            Some((
                self.cell_index,
                Vec2::<f32>::new(self.proj_x, self.proj_x * self.radians.tan()),
            ))
        } else {
            None
        }
    }
}

impl Grid2 {
    fn new(bound: Bound2<i32>) -> Self {
        Grid2 { bound: bound }
    }

    fn iter(&self, pos: Vec2<f32>, radians: f32) -> DDAIterator {
        DDAIterator::new(pos, radians, &self.bound)
    }
}

#[cfg(test)]
mod tests {
    use super::{Bound2, Grid2, Vec2};
    #[test]
    fn dda_test() {
        let min = Vec2::<i32>::new(0, 0);
        let max = Vec2::<i32>::new(64, 64);
        let b = Bound2::<i32>::new(min, max);
        let g = Grid2::new(b);
        let pos = Vec2::<f32>::new(22.5_f32, 1.0_f32);
        g.iter(pos, std::f32::consts::FRAC_PI_4 * 3f32)
            .for_each(|v| println!("{:?}", v));
    }
}
