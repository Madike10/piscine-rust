#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;

    fn append_number(&mut self, nb_to_append: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self{
         self.value.push_str(&str_to_append);
         self.clone()    
    }
    fn append_number(&mut self, nb_to_append: f64) -> Self{
        let nb_to_append_str = nb_to_append.to_string();
        self.value.push_str(&nb_to_append_str);
        self.clone()
    }
    fn remove_punctuation_marks(&mut self) -> Self{
        self.value.retain(|c| c.is_ascii_alphanumeric() || c == ' ');
        self.clone()
    }

}



fn main() {
    let mut str_aux = StringValue {
        value: String::from("hello"),
    };
    println!("Before append: {}", str_aux.value);

    str_aux.append_str(String::from(" there!"));
    println!("After append: {}", str_aux.value);

    str_aux.remove_punctuation_marks();
    println!("After removing punctuation: {}", str_aux.value);

}
