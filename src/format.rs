use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum VarValue {
    Literal(String),
    CommandResult(String),
    Cores()
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Step {
    Run(String),
    Info(String),
    Confirm(String, Vec<Step>, Vec<Step>),
    ConfirmFile(String, Vec<Step>, Vec<Step>),
    Variable(String, VarValue),
    Cancel
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    name: String,
    provides: String,
    steps: Vec<Step>
}