pub enum Grade {
    Lower,
    Medium,
    Higher
}

#[derive(PartialEq)]
pub enum EducationalStage {
    HighSchool {name: String},
    College {program: String}
}

impl EducationalStage {
    pub fn same_variant(&self, educational_stage: &EducationalStage) -> bool {
        match self {
            EducationalStage::HighSchool {..} => {
                if let EducationalStage::HighSchool {..} = educational_stage {
                    true
                } else {
                    false
                }
            },
            EducationalStage::College {..} => {
                if let EducationalStage::College {..} = educational_stage {
                    true
                } else {
                    false
                }
            },
        }
    }
}

pub struct Student <'a> {
    name: String,
    grade: Grade,
    educational_stage: &'a EducationalStage
}

impl <'a> Student <'a> {
    pub fn new(name: String, grade: Grade, educational_stage: &'a EducationalStage) -> Student {
        Student {name, grade, educational_stage}
    }
}

pub struct Class <'a> {
    name: String,
    professor_name: String, 
    students: Vec<Student<'a>>      
}

impl <'a> Class <'a> {
    pub fn new(name: String, professor_name: String) -> Class<'a> {
        Class {name, professor_name, students: Vec::new()}
    }

    pub fn enrroll_student(&mut self, student: Student<'a>) -> Result<(), &'static str> {
        let educational_stage = self.students.get(0).map(|s| s.educational_stage);
        
        match educational_stage {
            Some(es) if !es.same_variant(student.educational_stage) => {
                Err("Different Educational Stage")                 
            },
            _ => {
                self.students.push(student);
                Ok(())
            }
        }            
    }

    pub fn same_institution(self) -> bool {
        if self.students.is_empty() {
            return false;
        }

        let educational_stage = self.students[0].educational_stage;

        self.students.iter()
            .map(|student| student.educational_stage)
            .all(|es| es == educational_stage)
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn enroll_student_ok() {
        let highschool= EducationalStage::HighSchool { name: String::from("Escuela 1") };
        let highschool_2= EducationalStage::HighSchool { name: String::from("Escuela 2") };
        let student_1 = Student::new(String::from("Student 1"), Grade::Lower, &highschool);
        let student_2 = Student::new(String::from("Student 2"), Grade::Higher, &highschool_2);

        let mut class = Class::new(String::from("Class 1"), String::from("Professor 1"));
        let result_1 = class.enrroll_student(student_1);
        let result_2 = class.enrroll_student(student_2);

        assert!(result_1.is_ok());
        assert!(result_2.is_ok());
        assert_eq!(2, class.students.len());
    }

    #[test]
    fn enroll_student_error() {
        let highschool= EducationalStage::HighSchool { name: String::from("Escuela 1") };
        let college = EducationalStage::College { program: String::from("Program 1") };
        let student_1 = Student::new(String::from("Student 1"), Grade::Lower, &highschool);
        let student_2 = Student::new(String::from("Student 2"), Grade::Lower, &college);

        let mut class = Class::new(String::from("Class 1"), String::from("Professor 1"));
        let result_1 = class.enrroll_student(student_1);
        let result_2 = class.enrroll_student(student_2);

        assert!(result_1.is_ok());
        assert!(result_2.is_err());
        assert_eq!(1, class.students.len());
    }

    #[test]
    fn same_institution_true() {
        let highschool= EducationalStage::HighSchool { name: String::from("Escuela 1") };
        let student_1 = Student::new(String::from("Student 1"), Grade::Lower, &highschool);
        let student_2 = Student::new(String::from("Student 2"), Grade::Higher, &highschool);

        let mut class = Class::new(String::from("Class 1"), String::from("Professor 1"));
        let _ = class.enrroll_student(student_1);
        let _ = class.enrroll_student(student_2);

        assert!(class.same_institution());
    }

    #[test]
    fn same_institution_false() {
        let highschool= EducationalStage::HighSchool { name: String::from("Escuela 1") };
        let highschool_2= EducationalStage::HighSchool { name: String::from("Escuela 2") };
        let student_1 = Student::new(String::from("Student 1"), Grade::Lower, &highschool);
        let student_2 = Student::new(String::from("Student 2"), Grade::Higher, &highschool_2);

        let mut class = Class::new(String::from("Class 1"), String::from("Professor 1"));
        let _ = class.enrroll_student(student_1);
        let _ = class.enrroll_student(student_2);

        assert!(!class.same_institution());
    }
}