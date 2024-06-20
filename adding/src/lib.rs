

// Create the function add_curry, which returns a closure. 
// The purpose is to 'curry' the add method to create more variations.
// In the adding crate or module where add_curry will reside

pub fn add_curry(base: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| base + x
}

// fn main() {
//     let add10 = add_curry(-10);
//     let add20 = add_curry(2066);
//     let add30 = add_curry(300000);

//     println!("{}", add10(5));
//     println!("{}", add20(195));
//     println!("{}", add30(5696));
// }
