use std::fs::File;


pub fn open_file(s: &str) -> File {
    File::open(s).unwrap_or("File not found".to_string());
}


// fn main() {
//     let filename = "created.txt";
//     File::create(filename).unwrap();

//     let a = open_file(filename);
//     println!("{:?}", a);
    
//     fs::remove_file(filename).unwrap();

//     //It must panic
//     let b = open_file(filename);
// }
