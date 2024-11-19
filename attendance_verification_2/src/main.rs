#[derive(Debug)]
enum GradeLevel{
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major{
    ComputerScience,
    ElectricalEngineering,
}

#[derive(Debug)]

struct Student {
    grade: GradeLevel,
    name: String,
    major: Major,
}

impl Student {
    fn new(name:String, grade:GradeLevel, major:Major) -> Self {
        Student {
            grade: grade,
            name: name,
            major: major,
        }
    }

    fn introduce_yourself(&self) {

        let grade_msg = match self.grade {
            GradeLevel::Bachelor => "I am a Bachelor student",
            GradeLevel::Master => "I am a Masters student",
            GradeLevel::PhD => "I am a PhD student",
        };
        let major_msg = match self.major {
            Major::ComputerScience => "I am a Computer Science major",
            Major::ElectricalEngineering => "I am an Electrical Engineering Student"
        };
        println!("My name is {:?}. {:?}. {:?}.",self.name, major_msg, grade_msg);
    }
}

fn main() {
    let s1 = Student::new("Sarah Kolia".to_string(), GradeLevel::Bachelor, Major::ComputerScience);
    s1.introduce_yourself();
}