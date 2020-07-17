use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Clone)]
pub struct Student {
    pub name: String,
    pub grade: Grade,
    pub educational_stage: Rc<EducationalStage>
}

impl Student {
    pub fn new(name: String, grade: Grade, educational_stage: Rc<EducationalStage>) -> Student {
        Student {name, grade, educational_stage}
    }
}

pub struct Class {
    pub _name: String,
    pub _professor_name: String, 
    pub students: Vec<Rc<RefCell<Student>>>      
}

impl Class {
    pub fn new(_name: String, _professor_name: String) -> Class {
        Class {_name, _professor_name, students: Vec::new()}
    }

    pub fn enrroll_student(&mut self, student: Rc<RefCell<Student>>) -> Result<(), &'static str> {
        let first_student = self.students.get(0);
        
        match first_student {
            Some(es) if !es.borrow().educational_stage.same_variant(student.borrow().educational_stage.as_ref()) => {
                Err("Different Educational Stage")                 
            },
            _ => {
                self.students.push(student);
                Ok(())
            }
        }            
    }

    pub fn same_institution(&self) -> bool {
        if self.students.is_empty() {
            return false;
        }

        let first_student = self.students[0].borrow();
        let educational_stage = first_student.educational_stage.as_ref();

        let compare_es = |es0: &Rc<RefCell<Student>>| {
            es0.borrow().educational_stage.as_ref() == educational_stage
        };

        self.students.iter()
            .all(compare_es)
    }

    pub fn into_iter_ordered(&self) -> impl Iterator<Item = Rc<RefCell<Student>>> {
        let mut temp_students = self.students.clone();
        temp_students.sort_by_key(|student| Reverse(student.borrow().grade));
        temp_students.into_iter()
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn enroll_student_ok() {
        let highschool= EducationalStage::HighSchool { name: String::from("Escuela 1") };
        let highschool_2= EducationalStage::HighSchool { name: String::from("Escuela 2") };
        let student_1 = Rc::new(RefCell::new(Student::new(String::from("Student 1"), Grade::Lower, Rc::new(highschool))));
        let student_2 = Rc::new(RefCell::new(Student::new(String::from("Student 2"), Grade::Higher, Rc::new(highschool_2))));

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
        let student_1 = Rc::new(RefCell::new(Student::new(String::from("Student 1"), Grade::Lower, Rc::new(highschool))));
        let student_2 = Rc::new(RefCell::new(Student::new(String::from("Student 2"), Grade::Lower, Rc::new(college))));

        let mut class = Class::new(String::from("Class 1"), String::from("Professor 1"));
        let result_1 = class.enrroll_student(student_1);
        let result_2 = class.enrroll_student(student_2);

        assert!(result_1.is_ok());
        assert!(result_2.is_err());
        assert_eq!(1, class.students.len());
    }

    #[test]
    fn same_institution_true() {
        let highschool= Rc::new(EducationalStage::HighSchool { name: String::from("Escuela 1") });
        let student_1 = Rc::new(RefCell::new(Student::new(String::from("Student 1"), Grade::Lower, Rc::clone(&highschool))));
        let student_2 = Rc::new(RefCell::new(Student::new(String::from("Student 2"), Grade::Higher, Rc::clone(&highschool))));

        let mut class = Class::new(String::from("Class 1"), String::from("Professor 1"));
        let _ = class.enrroll_student(student_1);
        let _ = class.enrroll_student(student_2);

        assert!(class.same_institution());
    }

    #[test]
    fn same_institution_false() {
        let highschool= EducationalStage::HighSchool { name: String::from("Escuela 1") };
        let highschool_2= EducationalStage::HighSchool { name: String::from("Escuela 2") };
        let student_1 = Rc::new(RefCell::new(Student::new(String::from("Student 1"), Grade::Lower, Rc::new(highschool))));
        let student_2 = Rc::new(RefCell::new(Student::new(String::from("Student 2"), Grade::Higher, Rc::new(highschool_2))));

        let mut class = Class::new(String::from("Class 1"), String::from("Professor 1"));
        let _ = class.enrroll_student(student_1);
        let _ = class.enrroll_student(student_2);

        assert!(!class.same_institution());
    }

    #[test]
    fn students_ordered() {
        let highschool= Rc::new(EducationalStage::HighSchool { name: String::from("Escuela 1") });
        let student_1 = Rc::new(RefCell::new(Student::new(String::from("Student 1"), Grade::Lower, Rc::clone(&highschool))));
        let student_2 = Rc::new(RefCell::new(Student::new(String::from("Student 2"), Grade::Higher, Rc::clone(&highschool))));
        let student_3 = Rc::new(RefCell::new(Student::new(String::from("Student 2"), Grade::Medium, Rc::clone(&highschool))));

        let mut class = Class::new(String::from("Class 1"), String::from("Professor 1"));
        let _ = class.enrroll_student(student_1);
        let _ = class.enrroll_student(student_2);
        let _ = class.enrroll_student(student_3);

        let mut iter = class.into_iter_ordered();

        assert_eq!(Grade::Higher, iter.next().unwrap().borrow().grade);
        assert_eq!(Grade::Medium, iter.next().unwrap().borrow().grade);
        assert_eq!(Grade::Lower, iter.next().unwrap().borrow().grade);
        assert_eq!(Grade::Lower, class.students[0].borrow().grade);
        assert_eq!(Grade::Higher, class.students[1].borrow().grade);
        assert_eq!(Grade::Medium, class.students[2].borrow().grade);
    }
}