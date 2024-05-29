pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())

}

pub fn str_function(a: String) -> (String, String) {
    let mut result = String::new();
    let mut index = 0;
    for char in a.chars() {
        let temp = char.to_string().parse::<f64>().unwrap().exp();
      result.push_str(&temp.to_string());
      while index <= a.len() {
        result.push(' ');
        index += 1;
      }
      break;
    }
    (a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut result : Vec<f64> = Vec::new();
    for num in &b {
      result.push((*num as f64).ln());
    }
    (b, result)
}

fn main() {
    let a: i32 = 0;
    let b = String::from("1 2 4 5 6");
    let c = vec![1, 2, 4, 5];

    println!("{:?}", nbr_function(a));
    println!("{:?}", str_function(b));
    println!("{:?}", vec_function(c));
}

