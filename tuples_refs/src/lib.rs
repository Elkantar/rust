#[derive(Debug)]
pub struct Student(pub u32, pub String, pub String);

// Define functions to access id, first_name, and last_name
pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    student.1.clone()
}

pub fn last_name(student: &Student) -> String {
    student.2.clone()
}