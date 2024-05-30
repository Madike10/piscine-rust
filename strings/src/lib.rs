pub fn char_length(s: &str) -> usize {
    s.chars().count()
}



fn main() {
	println!("length of {} = {}", "❤", char_length("❤"));
	println!("length of {} = {}", "形声字", char_length("形聲字"));
	println!("length of {} = {}", "change", char_length("change"));
	println!("length of {} = {}", "😍", char_length("😍"));
}