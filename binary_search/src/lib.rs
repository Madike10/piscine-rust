pub fn binary_search(sorted_list: &[i32], target: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = sorted_list.len();

    while start < end {
        let mid = start + (end - start) / 2;

        if sorted_list[mid] == target {
            return Some(mid);
        } else if sorted_list[mid] < target {
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    None
}


// fn main() {
//     let sorted_list = vec![1, 3, 5, 7, 9, 11, 13];
//     let target = 7;
    
//     match binary_search(&sorted_list, target) {
//         Some(index) => println!("{} found at index {}", target, index),
//         None => println!("{} not found in the list", target),
//     }
// }

// And its output:

// $ cargo run
// 7 found at index 3
