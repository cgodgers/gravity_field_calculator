use std::ops::Sub;
use std::ops::Add;
use std::fmt::Display;
use std::fmt;


pub const G: f32 = 6.67428e-11;

#[derive(Debug)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}


impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:e}, {:e})", self.x, self.y)
    }
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
	type Output = Point;
	fn sub(self, other: &'b Point) -> Point {
		Point {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}
impl<'a, 'b> Add<&'b Point> for &'a Point {
	type Output = Point;
	fn add(self, other: &'b Point) -> Point {
		Point {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}

}

impl Point {
	pub fn new(x: f32, y: f32) -> Point {
		Point { x, y }
	}
	fn distance_from(&self, p: &Point) -> f32 {
		let difference = self - p;

		let distance = (difference.x.powi(2) + difference.y.powi(2)) as f32;

		distance.sqrt()
	}

	fn unitarize(&mut self) {
		let length = ((self.x.powi(2) + self.y.powi(2)) as f32).sqrt();

		self.x = self.x / length;
		self.y = self.y / length;
	}


}


#[derive(Debug)]
pub struct Mass {
	point: Point,
	weight: i32,
}

impl Mass {

	//TODO: generalitzar el tipus.
	pub fn new(x: f32, y: f32, weight: i32) -> Mass {
		Mass { point: Point::new(x, y), weight }
	}

	pub fn get_gravity_field(&self, p: &Point) -> Point {
		let distance = self.point.distance_from(p);

		let mut vector = p - &self.point;

		vector.unitarize();

		let module = - G * (self.weight as f32) / distance.powi(2);

		Point::new(
			module * vector.x,
			module * vector.y,
		)

		//G * (self.weight as f32) / distance.powi(2)
	}



}

