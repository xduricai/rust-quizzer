use std::fs;
use serde_json::to_string;
use crate::error::Error;
use crate::question::Question;

pub fn save_questions(questions: Vec<Question>) -> Result<(), Error> {
    let questions_json: String = to_string(&questions)?;
    fs::write("./questions.json", questions_json)?;
    Ok(())
}