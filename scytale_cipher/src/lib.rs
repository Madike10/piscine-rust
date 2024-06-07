pub fn scytale_cipher(message: String, i: u32) -> String {
    if i == 0 {
        return message;
    }

    let message_len = message.len() as u32;
    let num_lines = (message_len as f64 / i as f64).ceil() as u32;
    let total_chars = (num_lines * i) as usize;

    let mut copy_message = message.clone();
    while copy_message.len() < total_chars {
        copy_message.push(' ');
    }
    let mut result = String::new();

    for col in 0..i {
        for row in 0..num_lines {
            let index = (row * i + col) as usize;
            if let Some(c) = copy_message.chars().nth(index) {
                result.push(c);
            }
        }
    }

    result.trim().to_string()
}

// fn main() {
//     println!("\"scytale Code\" size=6 -> {:?}", scytale_cipher(String::from("scytale Code"), 6));
//     println!("\"scytale Code\" size=8 -> {:?}", scytale_cipher(String::from("scytale Code"), 8));
// }