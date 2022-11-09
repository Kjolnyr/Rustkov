use crate::brain_prelude::*;

#[derive(Debug, Clone)]
pub enum SentenceDirection {
    Backward,
    Forward,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SentenceMarker {
    Placeholder,
    Start,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateElement {
    Marker(SentenceMarker),
    Word(String),
}
