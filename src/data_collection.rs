use crate::traits::Collect;
use crate::models::topic::Topic;
use crate::models::student::Student;

pub struct DataCollection;

impl DataCollection {
    pub fn collect_students() -> Vec<Student> {
        Student::collect_input()
    }

    pub fn collect_topics() -> Vec<Topic> {
        Topic::collect_input()
    }
}