#[derive(Debug)]
pub struct Circle {
	pub center  : Point,
	pub radius  : f64,
}

impl Circle {
    pub fn new(x : f64, y : f64, radius: f64) -> Self {
        Circle {
            center: Point{x , y},
            radius: radius,
        }
    }
    pub fn diameter(&self) -> f64 {
         self.radius * 2.0
    }
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    pub fn intersect(&self, c2 : &Self) -> bool{
        self.center.distance(&c2.center) < self.radius + c2.radius
    }

}
#[derive(Debug)]

pub struct Point {
   pub x : f64,
   pub y : f64,
}

impl Point {
    pub fn new (x : f64, y : f64) -> Self {
        Point {
            x : x,
            y : y,
        }
    }
    pub fn distance (&self, p : &Self) -> f64{
        let dx = self.x - p.x ;
        let dy = self.y - p.y ;
        (dx * dx + dy * dy).sqrt()
        
    }
}


fn main() {
	let circle = Circle::new(500.0, 500.0, 150.0);
	let circle1 = Circle {
		center: Point { x: 80.0, y: 115.0 },
		radius: 30.0,
	};
	let point_a = Point { x: 1.0, y: 1.0 };
	let point_b = Point { x: 0.0, y: 0.0 };
	println!("circle = {:?} area = {}", circle, circle.area());
	println!("circle = {:?} diameter = {}", circle, circle.diameter());
	println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
	println!(
		"circle and circle1 intersect = {}",
		circle.intersect(&circle1)
	);

	println!(
		"distance between {:?} and {:?} is {}",
		point_a,
		point_b,
		point_a.distance(&point_b)
	);

}