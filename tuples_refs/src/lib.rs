#[derive(Debug)]
pub struct Student (pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
    Student.0

}

pub fn first_name(student: &Student) -> String {
    Student.1.to_string()
}

pub fn last_name(student: &Student) -> String {
    Student.2.to_string()
}
