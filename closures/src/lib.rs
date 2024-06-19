fn first_fifty_even_square() -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..50 {
        println!("{}", i);
        result.push(i * i * 2);
    }
    result

}

fn main() {
	println!("Hello, world!");
	let v1 = first_fifty_even_square();

	println!("All elements in {:?}, len = {}", v1, v1.len());
}
