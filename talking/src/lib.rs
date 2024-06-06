pub fn talking(text: &str) -> &str {
    if (text.chars().all(check_all_upper)) && (text.chars().last() == Some('!')){
        return &("There is no need to yell, calm down!");
    }else if (!text.chars().all(check_all_upper)) && (text.chars().last() == Some('!')){
        return &("Sure.");
    }else if (text.chars().all(check_all_upper)) && (text.chars().last() == Some('?')){
        return &("Quiet, I am thinking!");

    }else if text == ""{
        return &("Just say something!");
    }else{
        return &("Interesting");
    }
}

fn check_all_upper(c : char) -> bool{
    c.is_uppercase() || !c.is_alphabetic()
}

fn main() {
    println!("{:?}", talking("JUST DO IT!"));
    println!("{:?}", talking("Hello how are you?"));
    println!("{:?}", talking("WHAT'S GOING ON?"));
    println!("{:?}", talking("something"));
    println!("{:?}", talking(""));
}