pub fn first_subword( mut s: String) -> String {
    let mut res = String::new();
    let mut pos = 0;
    for val in s.chars(){
        if pos != 0 && (val.is_uppercase() || val == '_'){
        break
        }else{
            res.push(val);
        }
        pos += 1
    }
    return res ;
}


fn main() {
	let s1 = String::from("helloWorld");
	let s2 = String::from("snake_case");
	let s3 = String::from("CamelCase");
	let s4 = String::from("just");

	println!("first_subword({}) = {}", s1.clone(), first_subword(s1));
	println!("first_subword({}) = {}", s2.clone(), first_subword(s2));
	println!("first_subword({}) = {}", s3.clone(), first_subword(s3));
	println!("first_subword({}) = {}", s4.clone(), first_subword(s4));
}
