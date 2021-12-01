#![allow(dead_code)]
#![allow(unused_variables)]
use std::{f32::{EPSILON, consts::FRAC_PI_2}, ops::Mul, process::Output};

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

impl<T> Mul for &Vec2<T> where T:Primitive{
    type Output = Vec2<T>;
    fn mul(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2::<T>{x:self.x * rhs.x, y:self.y*rhs.y}
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
    txty:Vec2<f32>,
    dxdy:Vec2<f32>,
    rxry:Vec2<f32>,
    cell_index: Vec2<i32>,
    bound: &'a Bound2<i32>,
}

impl<'a> DDAIterator<'a> {
    fn new(pos: Vec2<f32>, radians: f32, bound: &'a Bound2<i32>) -> Self {
        let rx = radians.cos();
        let ry = radians.sin();
        let rxry = Vec2::<f32>::new(rx,ry);
        let mut txty = Vec2::<f32>::new(0f32,0f32);
        let mut dxdy = Vec2::<f32>::new(0f32,0f32);
        if rx < 0f32{
            dxdy.x =-1f32/rx;
            txty.x = ((pos.x.floor()) - pos.x) / rx;
        }else{
            dxdy.x = 1f32/rx;
            txty.x = ((pos.x.floor() + 1f32) - pos.x) / rx;
        }
        if ry < 0f32{
            dxdy.y = -1f32/ry;
            txty.y = ((pos.y.floor()) - pos.y) / ry;
        }else{
            dxdy.y = 1f32/ry;
            txty.y = ((pos.y.floor() + 1f32) - pos.y) / ry;
        }
        let cell_index = Vec2::<i32> { x: pos.x as i32, y: pos.y as i32 };
        DDAIterator {
            txty:txty,
            dxdy:dxdy,
            rxry:rxry,
            cell_index:cell_index,
            bound: bound,
        }
    }
}

impl<'a> Iterator for DDAIterator<'a> {
    type Item = (Vec2<i32>, f32);
    fn next(&mut self) -> Option<Self::Item> {
        let t;
        if self.txty.x < self.txty.y{
            t = self.txty.x;
            self.txty.x += self.dxdy.x;
            if self.rxry.x < 0f32{
                self.cell_index.x -= 1;
            }else{
                self.cell_index.x +=1
            }
        }else if self.txty.y < self.txty.x{
            t = self.txty.y;
            self.txty.y += self.dxdy.y;
            if self.rxry.y < 0_f32{
                self.cell_index.y -= 1;
            }else{
                self.cell_index.y += 1;
            }
        }else{
            t = self.txty.x;
            self.txty.y += self.dxdy.y;
            self.txty.x += self.dxdy.x;
            if self.rxry.y < 0_f32{
                self.cell_index.y -= 1;
            }else{
                self.cell_index.y += 1;
            }
            if self.rxry.x < 0f32{
                self.cell_index.x -= 1;
            }else{
                self.cell_index.x +=1
            }

        }
        if (self.cell_index.x >= self.bound.min.x && self.cell_index.x < self.bound.max.x)
            && (self.cell_index.y >= self.bound.min.y && self.cell_index.y < self.bound.max.y)
        {
            Some((
                self.cell_index,
                t,
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
        let pos = Vec2::<f32>::new(0.5f32,0f32);
        g.iter(pos, std::f32::consts::FRAC_PI_4 * 3f32 )
            .for_each(|v| println!("{:?}", v));
    }
}
