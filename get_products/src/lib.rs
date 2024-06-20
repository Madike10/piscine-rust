// Instructions

// Create a function named get_products that takes a vector of integers, and returns a vector of the products of each index.

// You'll need to return the product of every index except the current one.
// Example:

// For [1,2,3,4], we get:

//     for the number 1 we get 2*3*4 = 24.
//     for the number 3 we get 1*2*4 = 8.

// Expected functions

pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut result = Vec::new();
    for (index, value) in arr.iter().enumerate() {
        let mut product = 1;
        for (index2, value2) in arr.iter().enumerate() {
            if index!= index2 {
                product *= *value2;
            }
        }
        result.push(product);
    }
    return result;

}

fn main() {
    let arr: Vec<usize> = vec![1, 7, 3, 4];
    let output = get_products(arr);
    println!("{:?}", output);
}

