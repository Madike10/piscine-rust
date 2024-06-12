pub fn identity<T>(v: T) -> T {
    v
}
fn main() {
	println!("{}", identity("Hello, world!"));
	println!("{}", identity(3));
}