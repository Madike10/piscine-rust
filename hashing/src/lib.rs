use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &mut [i32]) -> f64 {
    list.sort_unstable();
    let len = list.len();
    if len % 2 == 0 {
        (list[len / 2 - 1] + list[len / 2]) as f64 / 2.0
    } else {
        list[len / 2] as f64
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for &num in list {
        *map.entry(num).or_insert(0) += 1;
    }
    map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
        .unwrap_or(0)
}




// fn main() {
// 	println!("Hello, world!");
// 	let v = vec![4, 7, 5, 2, 5, 1, 3];
// 	println!("mean {}", hashing::mean(&v));
// 	println!("median {}", hashing::median(&v));
// 	println!("mode {}", hashing::mode(&v));
// }

// And its output;

// $ cargo run
// mean 3.857142857142857
// median 4
// mode 5
// $
