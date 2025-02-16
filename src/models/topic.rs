use std::io::{self, Write};
use crate::traits::Collect;
use crate::enums::difficulty::Difficulty;

#[derive(Debug, Clone)]
pub struct Topic {
    id: u32,
    title: String,
    difficulty: Difficulty,
}

impl Topic {
    pub fn new(id: u32, title: String, difficulty: Difficulty) -> Self {
        Self { id, title, difficulty }
    }
}

impl Collect for Topic {
    fn collect_input() -> Vec<Self> {
        let mut topics = Vec::new();
        let mut id = 1;

        loop {
            print!("Enter topic title (or 'done' to finish): ");
            io::stdout().flush().unwrap();
            let mut title = String::new();
            io::stdin().read_line(&mut title).unwrap();
            let title = title.trim().to_string();

            if title.to_lowercase() == "done" {
                break;
            }

            print!("Enter topic difficulty (Easy, Medium, Hard): ");
            io::stdout().flush().unwrap();
            let mut difficulty = String::new();
            io::stdin().read_line(&mut difficulty).unwrap();
            let difficulty = match difficulty.trim().to_lowercase().as_str() {
                "easy" => Difficulty::Easy,
                "medium" => Difficulty::Medium,
                "hard" => Difficulty::Hard,
                _ => {
                    println!("Invalid difficulty. Please enter Easy, Medium, or Hard.");
                    continue;
                }
            };

            topics.push(Topic::new(id, title, difficulty));
            id += 1;
        }

        topics
    }
}