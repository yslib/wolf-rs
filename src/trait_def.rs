
pub trait Primitive
: Send + Copy + Sized + Clone + PartialOrd + PartialEq + Default {}

impl Primitive for bool {}

impl Primitive for f32{}

impl Primitive for u8{}


pub trait FromRef<T>{
    fn from_ref(t:&T)->Self;
}

impl FromRef<u8> for f32{
    fn from_ref(t:&u8)->f32{
        *t as f32
    }
}

