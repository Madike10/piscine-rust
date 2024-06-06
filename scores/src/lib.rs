pub fn score(letter: &str) -> u64 {
    let mut sum = 0;
    let letter_upper = letter.to_uppercase();
    for val in letter_upper.chars(){
        match val {
            'A' | 'E'| 'I'| 'O'| 'U'| 'L'| 'N'| 'R'| 'S'| 'T' => sum += 1,
            'D'|'G' => sum += 2,
            'B'| 'C'| 'M' | 'P' => sum += 3,
            'F'| 'H'| 'V'| 'W'| 'Y' => sum += 4,
            'K' => sum += 5,
            'J' | 'X' => sum += 8,
            'Q' | 'Z' => sum += 10,
            _ => sum += 0,
        }
    
    }
    return sum ;
}

fn main() {
    println!("{}", score("a"));
    println!("{}", score("ã ê Á?"));
    println!("{}", score("ThiS is A Test"));
}