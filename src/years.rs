pub mod year_2025;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum Part {
    One,
    Two,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Input {
    text: String,
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        Self { text: value }
    }
}
