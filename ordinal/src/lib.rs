pub fn num_to_ordinal(x: u32) -> String {
  let mut res = String::new();
  match x {
    1 => res.push_str("1st"),
    2 => res.push_str("2nd"),
    3 => res.push_str("3rd"),
    11 => res.push_str("11th"),
    12 => res.push_str("12th"),
    13 => res.push_str("13th"),
    _ => res.push_str(&format!("{}th", x)),
  }
  return res;
}

fn main() {
    println!("{}", num_to_ordinal(1));
    println!("{}", num_to_ordinal(22));
    println!("{}", num_to_ordinal(43));
    println!("{}", num_to_ordinal(47));
}