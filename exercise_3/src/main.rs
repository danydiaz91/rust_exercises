use exercise_3::*;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // HighSchool
    let highschool = Rc::new(EducationalStage::HighSchool {name: String::from("HighSchool 1")});
    let studenth1 = Rc::new(RefCell::new(Student::new(String::from("Student HS 1"), Grade::Medium, Rc::clone(&highschool))));
    let studenth2 = Rc::new(RefCell::new(Student::new(String::from("Student HS 2"), Grade::Lower, Rc::clone(&highschool))));
    let studenth3 = Rc::new(RefCell::new(Student::new(String::from("Student HS 3"), Grade::Higher, Rc::clone(&highschool))));
    let mut class1 = Class::new(String::from("Class 1"), String::from("Professor 1"));

    let _ = class1.enrroll_student(studenth1.clone());
    let _ = class1.enrroll_student(studenth2.clone());
    let _ = class1.enrroll_student(studenth3.clone());

    // College
    let college1 = Rc::new(EducationalStage::College {program: String::from("Program 1")});
    let college2 = Rc::new(EducationalStage::College {program: String::from("Program 2")});
    let studentc1 = Rc::new(RefCell::new(Student::new(String::from("Student HS 1"), Grade::Lower, Rc::clone(&college1))));
    let studentc2 = Rc::new(RefCell::new(Student::new(String::from("Student HS 2"), Grade::Medium, Rc::clone(&college1))));
    let studentc3 = Rc::new(RefCell::new(Student::new(String::from("Student HS 3"), Grade::Higher, college2)));
    let mut class2 = Class::new(String::from("Class 2"), String::from("Professor 2"));

    let _ = class2.enrroll_student(studentc1.clone());
    let _ = class2.enrroll_student(studentc2.clone());
    let _ = class2.enrroll_student(studentc3.clone());

    println!("\nAre the students of Class 1 from the same High School? {}", class1.same_institution());
    println!("Are the students of Class 2 from the same College? {}", class2.same_institution());

    println!("\nStudents Class 1:");
    class1.into_iter_ordered().for_each(|student| println!("Name: {}", student.borrow().name));

    println!("\nStudents Class 2:");
    class2.into_iter_ordered().for_each(|student| println!("Name: {}", student.borrow().name));

    println!("\nStudent {} is in the same program with the Student {}? {}", studentc1.borrow().name, studentc3.borrow().name, studentc1.borrow().educational_stage == studentc3.borrow().educational_stage);
    println!("Student {} is in the same program with the Student {}? {}", studentc1.borrow().name, studentc2.borrow().name, studentc1.borrow().educational_stage == studentc2.borrow().educational_stage);
}
