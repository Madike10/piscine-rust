pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();
    for i in input.chars(){
        let mut  _cipher : i8;
        if i.is_ascii_uppercase() {
            if key < 0 {
                _cipher = ((i as i8 - 65 + (26 - key)) % 26) + 65;
                res.push((_cipher as u8)as char)
            }
                _cipher = ((i as i8 - 65 + key) % 26) + 65;
                res.push((_cipher as u8)as char);
        }else if i.is_ascii_lowercase(){
            if key < 0 {
                _cipher = ((i as i8 - 97 + (26 - key)) % 26) + 97;
                res.push((_cipher as u8)as char);
            }
            _cipher = ((i as i8 - 97 + key) % 26) + 97;
            res.push((_cipher as u8)as char);
        }else{
            res.push(i);
        }
    }
    return res
}


fn main() {

    println!("The letter \"a\" becomes: {}", rotate("a", 26));
    println!("The letter \"m\" becomes: {}", rotate("m", 0));
    println!("The letter \"m\" becomes: {}", rotate("m", 13));
    println!("The letter \"a\" becomes: {}", rotate("a", 15));
    println!("The word \"MISS\" becomes: {}", rotate("MISS", 5));
    println!(
        "The decoded message is: {}",
        rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13)
    );
    println!(
        "The decoded message is: {}",
        rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5)
    );
    println!(
        "Your cypher wil be: {}",
        rotate("Testing with numbers 1 2 3", 4)
    );
    println!("Your cypher wil be: {}", rotate("Testing", -14));
    println!("The letter \"a\" becomes: {}", rotate("a", -1));
}
