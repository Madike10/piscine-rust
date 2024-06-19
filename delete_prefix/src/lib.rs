pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    if s.starts_with(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}


// use delete_prefix::delete_prefix;

fn main() {
	println!("{:?}", delete_prefix("abc", "abcdefghijklmnop"));
	println!("{:?}", delete_prefix("x", "abcdefghijklmnop"));
}