#[allow(dead_code)]
pub struct Car<'a> {
	pub plate_nbr: &'a str,
	pub model: &'a str,
	pub horse_power: u32,
	pub year: u32,
}

#[allow(dead_code)]
pub struct Truck<'a> {
	pub plate_nbr: &'a str,
	pub model: &'a str,
	pub horse_power: u32,
	pub year: u32,
	pub load_tons: u32,
}

pub trait Vehicle {
	fn model(&self) -> &str;
	fn year(&self) -> u32;
}

impl Vehicle for Truck<'_> {
    fn model(&self) -> &str {
        self.model
    }
    fn year(&self) -> u32 {
        self.year
    }
}

impl Vehicle for Car<'_> {
    fn model(&self) -> &str {
        self.model
    }
    fn year(&self) -> u32 {
        self.year
    }
}

pub fn all_models(list: Vec<&dyn Vehicle>) -> Vec<&str> {
    list.iter().map(|v| v.model()).collect()
}

fn main() {
    let vehicles: Vec<Box<dyn Vehicle>> = vec![
        Box::new(Car {
            plate_nbr: "A3D5C7",
            model: "Model 3",
            horse_power: 325,
            year: 2010,
        }),
        Box::new(Truck {
            plate_nbr: "V3D5CT",
            model: "Ranger",
            horse_power: 325,
            year: 2010,
            load_tons: 40,
        }),
    ];

    let vehicle_refs: Vec<&dyn Vehicle> = vehicles.iter().map(|v| v.as_ref()).collect();
    println!("{:?}", all_models(vehicle_refs));
}