use std::io::{self, Write};
use crate::traits::Collect;

#[derive(Debug, Clone)]
pub struct Student {
    id: u32,
    name: String,
}

impl Student {
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }
}

impl Collect for Student {
    fn collect_input() -> Vec<Self> {
        let mut students = Vec::new();
        let mut id = 1;

        loop {
            print!("Enter student name (or 'done' to finish): ");
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().to_string();

            if name.to_lowercase() == "done" {
                break;
            }

            students.push(Student::new(id, name));
            id += 1;
        }

        students
    }
}