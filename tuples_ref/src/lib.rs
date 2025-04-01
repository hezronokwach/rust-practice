#[derive(Debug)]
pub struct Student<'a>(pub u32, pub &'a str, pub &'a str);
pub fn id(student: &Student) -> u32 {
    student.0
}
pub fn first_name<'a>(student: &'a Student<'a>) -> &'a str {
    student.1
}

pub fn last_name<'a>(student: &'a Student<'a>) -> &'a str {
    student.2
}