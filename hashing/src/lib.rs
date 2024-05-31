use std::collections::HashMap;
pub fn mean(list: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for num in list {
        sum += num;
    }
    sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut list = list.clone();
    list.sort();
    if list.len() % 2 == 0 {
        (list[list.len() / 2] + list[list.len() / 2 - 1]) / 2
    } else {
        list[list.len() / 2]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in list {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut biggest = 0;
    for (_, value) in map {
        if value > biggest {
            biggest = value;
        }
    }
    for (key, value) in map {
        if value == biggest {
            return *key;
        }
    }
    return 0;
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
