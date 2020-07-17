use exercise_3::*;
use std::rc::Rc;

fn main() {
    // HighSchool
    let highschool = Rc::new(EducationalStage::HighSchool {name: String::from("HighSchool 1")});
    let studenth1 = Student::new(String::from("Student HS 1"), Grade::Medium, Rc::clone(&highschool));
    let studenth2 = Student::new(String::from("Student HS 2"), Grade::Lower, Rc::clone(&highschool));
    let studenth3 = Student::new(String::from("Student HS 3"), Grade::Higher, Rc::clone(&highschool));
    let mut class1 = Class::new(String::from("Class 1"), String::from("Professor 1"));

    let _ = class1.enrroll_student(&studenth1);
    let _ = class1.enrroll_student(&studenth2);
    let _ = class1.enrroll_student(&studenth3);

    // College
    let college1 = Rc::new(EducationalStage::College {program: String::from("Program 1")});
    let college2 = Rc::new(EducationalStage::College {program: String::from("Program 2")});
    let studentc1 = Student::new(String::from("Student HS 1"), Grade::Lower, Rc::clone(&college1));
    let studentc2 = Student::new(String::from("Student HS 2"), Grade::Medium, Rc::clone(&college1));
    let studentc3 = Student::new(String::from("Student HS 3"), Grade::Higher, college2);
    let mut class2 = Class::new(String::from("Class 2"), String::from("Professor 2"));

    let _ = class2.enrroll_student(&studentc1);
    let _ = class2.enrroll_student(&studentc2);
    let _ = class2.enrroll_student(&studentc3);

    println!("\nAre the students of Class 1 from the same High School? {}", class1.same_institution());
    println!("Are the students of Class 2 from the same College? {}", class2.same_institution());

    println!("\nStudents Class 1:");
    class1.into_iter_ordered().for_each(|student| println!("Name: {}", student.name));

    println!("\nStudents Class 2:");
    class2.into_iter_ordered().for_each(|student| println!("Name: {}", student.name));

    println!("\nStudent {} is in the same program with the Student {}? {}", studentc1.name, studentc3.name, studentc1.educational_stage == studentc3.educational_stage);
    println!("Student {} is in the same program with the Student {}? {}", studentc1.name, studentc2.name, studentc1.educational_stage == studentc2.educational_stage);
}
