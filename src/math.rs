
use super::trait_def::Primitive;

pub struct Vec3<T> where T:Primitive{
	x:T,
	y:T,
	z:T
}


impl<T> Vec3<T> where T:Primitive{
	pub fn new(x:T,y:T,z:T)->Self{
		Vec3::<T>{x,y,z}
	}
}

pub struct Vec2<T> where T:Primitive{
	x:T,
	y:T
}

impl<T> Vec2<T> where T:Primitive{
	pub fn new(x:T,y:T)->Self{
		Vec2::<T>{x,y}
	}
}

