pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (index, value) in array.iter().enumerate() {
        if *value == key {
            return Some(index);
        }
    }
    return None;
}
fn main() {
    let ar = [1, 3, 4, 6, 8, 9, 11];
    let f = search(&ar, 6);
    println!(
        "the element 6 is in the position {:?} in the array {:?}",
        f, ar
    );
}
