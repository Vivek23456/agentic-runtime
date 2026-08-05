use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum ProgrammingLanguage {
    Rust,
    Python,
    TypeScript,
    Go,
    Java,
    Custom,
}