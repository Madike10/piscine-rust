use std::fmt;

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub strength: f64,
	pub score: i32,
	pub money: i32,
	pub weapons: Vec<String>,
}


pub struct Fruit {
	pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player  {
	pub fn eat<T: Food>(&mut self, food: T) {
		self.strength += food.gives();
	}
   
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let first_lines = format!("{}", self.name);
      let second_line =   format!("Strength: {}, Score: {}, money: {}", self.strength, self.score, self.money);
      let third_line = format!("weapons: {:?}", self.weapons);
    write!(f, "{}\n{}\n{}", first_lines, second_line, third_line)
    }
}

pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * 4.0
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
      let proteine = self.weight_in_kg - (self.weight_in_kg * self.fat_content);
      let fat_c  = self.weight_in_kg - proteine;
      (proteine * 4.0) + (fat_c * 9.0)
    }
}


fn main() {
	let apple = Fruit { weight_in_kg: 1.0 };

	println!("this apple gives {} units of strength", apple.gives());

	let steak = Meat {
		weight_in_kg: 1.0,
		fat_content: 1.0,
	};

	let mut player1 = Player {
		name: String::from("player1"),
		strength: 1.0,
		score: 0,
		money: 0,
		weapons: vec![String::from("knife")],
	};
	println!("Before eating {:?}", player1);
	player1.eat(apple);
	println!("After eating an apple\n{}", player1);
	player1.eat(steak);
	println!("After eating a steak\n{}", player1);
}
