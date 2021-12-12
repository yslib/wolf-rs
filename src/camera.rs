use crate::math::{Vec2, normalize};

#[derive(Debug)]
pub struct WolfCamera {
	pub rotate_speed:f32,
	pub move_speed:f32,
	pub dir: Vec2<f32>,
    pub fov: f32,
    pub pos: Vec2<f32>,
}

impl Default for WolfCamera {
    fn default() -> Self {
        WolfCamera {
			rotate_speed:10f32,
			move_speed:1f32,
			dir:normalize(Vec2::new(1.0,1.0)),
            fov: 45f32,
            pos: Vec2::new(10f32, 32f32),
        }
    }
}

impl WolfCamera {
    pub fn new(pos:Vec2<f32>, dir:Vec2<f32>, fov:f32) -> Self {
		WolfCamera{
			rotate_speed:0.1f32,
			move_speed:0.1f32,
			dir:normalize(dir),
			fov:fov,
			pos:pos
		}
    }

	pub fn advance(&mut self, adv:i32){
		let advance = adv as f32 * self.move_speed;
		self.pos = self.pos + self.dir * advance;
	}

	pub fn rotate(&mut self, delta:f32){

		let old = self.get_view_angle();
		let angle = (delta as f32 * self.rotate_speed);

		// TODO:: replaced by matrix algebra later
		self.dir.x = angle.cos() * self.dir.x - angle.sin() * self.dir.y;
		self.dir.y = angle.sin() * self.dir.x + angle.cos() * self.dir.y;

		let invsqrt = (self.dir.x * self.dir.x + self.dir.y * self.dir.y).sqrt();
		self.dir = self.dir/invsqrt;

		println!("{:?}", self.dir);
		let new = self.get_view_angle();
	}

	pub fn get_view_angle(&self)->f32{
		self.dir.y.atan2(self.dir.x)
	}
}
