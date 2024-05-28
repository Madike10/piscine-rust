pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut v : Vec<String> = Vec::new();
    for name in names{
       let mut index = 0;
       let mut temp = String::new();
       let namesplit = name.split(" ");
       for n in namesplit {
        temp.push(n.chars().nth(0).unwrap());
        temp.push('.');
        if index == 0 {
            temp.push(' ');
            index +=1;
        }
       }
       v.push(temp);
    }
    return v;
}

fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
}

