pub fn capitalize_first(input: &str) -> String {
    let mut res = String::new();
    let mut pos = 0;
    for val in input.chars(){
        if pos == 0 {
            res.push(val.to_ascii_uppercase());
        }else{
            res.push(val.to_ascii_lowercase());
        }
        pos += 1
    }
    return res ;

}

pub fn title_case(input: &str) -> String {
    let mut res = String::new();
    let input_split = input.split(" ");
    for elem in input_split{
        res.push_str(&capitalize_first(elem));
        res.push(' ');
    }
    res.pop();
    return res;

}
    
pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for elem in input.chars() {
        if elem.is_uppercase() {
            res.push(elem.to_lowercase().next().unwrap());
        }else{
            res.push(elem.to_uppercase().next().unwrap());
        }
       
    }
    return res;

}
        
        
        
fn main() {
    println!("{}", capitalize_first(""));
    println!("{}", title_case("jill is leaving A"));
    println!("{}",change_case("heLLo THere"));
}

// And its output

// $ cargo run
// Joe is missing
// Jill Is Leaving A
// HEllO thERE
// $
