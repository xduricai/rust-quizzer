use serde::{Serialize, Deserialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub text: String,
    pub correct: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub text: String,
    pub answers: [Answer; 4]
}

impl Answer {
    pub fn new() -> Answer {
        Answer { text: String::from(""), correct: false }
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Question: {}\n", self.text)?;

        for i in 0..4 {
            writeln!(f, "\t{}) {}", i + 1, self.answers[i].text)?;
        }
        Ok(())
    }
}

//impl Display for Answer {
//    fn fmt(&self, f: mut std::fmt::Formatter<'_>) -> std::fmt::Result {    }
//}