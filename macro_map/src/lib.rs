// Create a macro rule called hash_map to initialize and declare a HashMap at the same time,
//  very similar to what vec! macro does for Vector.

// Your macro should accept both leading and non leading commas syntax to be more flexible in terms of coding style and reflect the language general style.

use std::collections::HashMap;
macro_rules! hash_map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

// fn main() {
//     let empty: HashMap<u32, u32> = hash_map!();
//     let new = hash_map!('a' => 22, 'b' => 1, 'c' => 10);
//     let nested = hash_map!(
//         "first" => hash_map!(
//             "Rob" => 32.2,
//             "Gen" => 44.1,
//             "Chris" => 10.0,
//         ),
//         "second" => hash_map!()
//     );
//     println!("{:?}", empty);
//     println!("{:?}", new);
//     println!("{:?}", nested);
// }
