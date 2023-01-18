#[derive(Debug, Clone, PartialEq)]
pub enum GameError {
    UnableToProcessFile(String),
    BadFile(String),
}
