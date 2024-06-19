// Using closures and iterators create a function, that returns the first 50 even numbers squared in a Vec<i32>.
pub fn first_fifty_even_square() -> Vec<i32> {
    (1..=100)
       .filter(|x| x % 2 == 0)
       .map(|x| x * x)
       .collect()
}

fn main() {
	println!("Hello, world!");
	let v1 = first_fifty_even_square();

	println!("All elements in {:?}, len = {}", v1, v1.len());
}
