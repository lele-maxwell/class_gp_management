
use std::rc::Rc;
use crate::models::topic::Topic;
use crate::models::student::Student;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Group {
    id: u32,
    label: String,
    topic: Rc<Topic>,
    students: Vec<Rc<Student>>,
}

impl Group {
    pub fn new(id: u32, label: String, topic: Rc<Topic>) -> Self {
        Self {
            id,
            label,
            topic,
            students: Vec::new(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn get_topic(&self) -> Rc<Topic> {
        Rc::clone(&self.topic)
    }

    pub fn get_students(&self) -> Vec<Rc<Student>> {
        self.students.clone()
    }

    pub fn generate_groups(topics: Vec<Rc<Topic>>, students: Vec<Rc<Student>>) -> Vec<Group> {
        let mut groups: Vec<Group> = Vec::new();
        let mut rng = rand::thread_rng();

        for (i, topic) in topics.iter().enumerate() {
            let group = Group::new(i as u32 + 1, format!("Group {}", i + 1), Rc::clone(topic));
            groups.push(group);
        }

        let mut students = students.clone();
        students.shuffle(&mut rng);

        for student in students {
            let group = groups.iter_mut().min_by_key(|g| g.students.len()).unwrap();
            group.students.push(Rc::clone(&student));
        }

        groups
    }
}