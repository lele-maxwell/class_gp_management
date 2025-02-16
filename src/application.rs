use std::rc::Rc;
use std::cell::RefCell;
use crate::{data_collection::DataCollection, models::{group::Group, topic::Topic, student::Student}, enums::labelling::Labelling};

#[derive(Debug)]
struct AppState {
    labelling: Labelling,
    groups: Rc<RefCell<Vec<Group>>>,  // Use RefCell for mutability
}

pub struct Application {
    state: AppState,
}

impl Application {
    // Constructor to initialize AppState
    pub fn new(labelling: Labelling) -> Self {
        Self {
            state: AppState {
                labelling,
                groups: Rc::new(RefCell::new(Vec::new())), // Initialize with an empty Vec
            },
        }
    }

    // Run the application
    pub fn run(&mut self) {
        println!("Running application");

        // Collecting topics and students
        let topics: Vec<Rc<Topic>> = DataCollection::collect_topics()
            .into_iter()
            .map(Rc::new)
            .collect();
        let students: Vec<Rc<Student>> = DataCollection::collect_students()
            .into_iter()
            .map(Rc::new)
            .collect();

        // Generate groups and update the state
        let groups = Group::generate_groups(topics, students);
        self.state.groups = Rc::new(RefCell::new(groups));

        // Print out details of the groups
        for group in self.state.groups.borrow().iter() {
            println!("Group ID: {}", group.get_id());
            println!("Group Label: {}", self.state.labelling.generate_label(group.get_id()));
            println!("Topic: {:?}", group.get_topic());
            println!("Students: {:?}", group.get_students());
        }
    }
}