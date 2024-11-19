mod major;
mod student;


fn main() {
    let student = student::Student::new("John", "CS");
    println!("{:?}", student);
}
