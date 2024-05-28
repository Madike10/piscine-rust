pub fn to_url(s: &str) -> String {
    s.replace(' ', "%20")
}


fn main() {
	let s = "Hello, world!";
	println!("{} to be use as an url is {}", s, to_url(s));
}

