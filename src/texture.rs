use crate::{trait_def::{Primitive}, canvas::Color};

pub struct Texture2D<T>
where
    T: Primitive
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

impl<T> Texture2D<T> where T:Primitive{
    pub fn buffer(&self)->&Vec<T> {
        &self.data
    }
	#[inline(always)]
	pub fn sample_nearest(&self, u:f32,v:f32)->T{
		let ix = (u * self.width as f32) as usize;
		let iy = (v * self.height as f32) as usize;
		let index= ix + iy * self.width as usize;
		self.data[index]
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
            data: data.into_iter().map(|u|T::from(u)).collect(),
            width: width,
            height: height,
            channel: channel,
        }
    }
}
