pub mod models {

    pub enum Grade {
        Lower,
        Medium,
        Higher
    }

    pub enum EducationalStage {
        HighSchool {name: String},
        College {program: String}
    }

    pub struct Student {
        name: String,
        grade: Grade,
        educational_stage: EducationalStage
    }

    pub struct Class {
        name: String,
        professor_name: String, 
        students: Vec<Student>        
    }
}