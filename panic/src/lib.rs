use std::fs::File;


pub fn open_file(s: &str) -> File {
    File::open(s).expect("File not found");
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
