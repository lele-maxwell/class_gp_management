mod application;
mod data_collection;
mod models;
mod traits;
mod enums;

use application::Application;
use enums::labelling::Labelling;

fn main() {
    let labelling = Labelling::Numeric; // Change this to the desired labelling format
    let mut app = Application::new(labelling);
    app.run();
}