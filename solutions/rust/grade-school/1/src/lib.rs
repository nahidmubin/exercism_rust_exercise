use std::collections::{HashMap, HashSet};

pub struct School {
    students: HashMap<String, u32>
}

impl School {
    pub fn new() -> School {
        Self { students: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.entry(student.to_string()).or_insert(grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let available_grades: HashSet<u32>  = self.students.values().copied().collect();
        let mut available_grades: Vec<u32> = available_grades.into_iter().collect();
        available_grades.sort();
        available_grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students: Vec<String> = self.students.iter().filter(|(_, v)| **v == grade).map(|(k, _)| k.clone()).collect();
        students.sort();
        students
    }
}
