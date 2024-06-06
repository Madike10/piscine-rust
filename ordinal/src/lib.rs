pub fn num_to_ordinal(x: u32) -> String {
  let mut res = x.to_string();
  if res == "11".to_string() || res == "12".to_string() || res == "13".to_string(){
    res.push_str("th");
    return res;
  }
  match res.chars().last(){
    Some('1') => res.push_str("st"),
    Some('2') => res.push_str("nd"),
    Some('3') => res.push_str("rd"),
    
    _ => res.push_str("th"),

  }
  return res;
}

fn main() {
    println!("{}", num_to_ordinal(1));
    println!("{}", num_to_ordinal(22));
    println!("{}", num_to_ordinal(43));
    println!("{}", num_to_ordinal(47));
}