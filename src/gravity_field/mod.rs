#![allow(dead_code)]
#![allow(unused_variables)]

mod structures;
use self::structures::*;

struct System {
	masses: Vec<Mass>,
	curr: Point,	
}

impl System {
	fn new(masses: Vec<Mass>, p: Point) -> System {
		System { masses, curr: p }
	}

	fn sum_gravity_fields(&self) -> Point {
		let mut vector = Point::new(0.0, 0.0);
		for mass in self.masses.iter() {
			vector = &vector + &mass.get_gravity_field(&self.curr);
		}
		vector
	}
}


pub fn run() {
	let masses = vec![
		Mass::new(3.0, 4.0, 100),
		Mass::new(3.0, 0.0, 200),
		Mass::new(0.0, 0.0, 500),
		Mass::new(0.0, -8.0, 1000),
	];

	let p = Point::new(6.0, 0.0);
	
	let system = System::new(masses, p);

	let gravity_field = system.sum_gravity_fields();

	println!("{}", gravity_field);

}