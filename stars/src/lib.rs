pub fn stars(n: u32) -> String {

    let mut res = String::new();
    for _ in 0..2_u32.pow(n) {
      res.push_str("*");
}
return  res;
}

// fn main() {
//     println!("{}", stars(1));
//     println!("{}", stars(4));
//     println!("{}", stars(5));
// }
