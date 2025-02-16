#[derive(Debug, Clone, Default)]
pub enum Labelling {
    #[default]
    Numeric,
    Alphabet,
    AlphabetNumeric,
}

impl Labelling {
    // Convert the enum variant to a string
    pub fn to_string(&self) -> &str {
        match self {
            Labelling::Numeric => "Numeric",
            Labelling::Alphabet => "Alphabet",
            Labelling::AlphabetNumeric => "AlphabetNumeric",
        }
    }

    // Generate a label based on the enum variant
    pub fn generate_label(&self, id: u32) -> String {
        match self {
            Labelling::Numeric => id.to_string(),
            Labelling::Alphabet => ((b'A' + (id - 1) as u8) as char).to_string(),
            Labelling::AlphabetNumeric => format!("G{}", id),
        }
    }
}