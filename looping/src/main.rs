use std::io;

fn main() {
    let mut i = 0;
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        if input.trim() == answer {
            println!("Number of trials: {}", i);
            break;
        }
        println!("{}", riddle);
        i += 1
    }
}
