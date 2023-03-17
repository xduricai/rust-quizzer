use std::fs;
use anyhow::Ok;
use serde_json::from_str;
use crate::question::Question;

pub fn load_questions() -> Result<Vec<Question>, anyhow::Error> {
    let questions_json = fs::read_to_string("./questions.json")?;
    let questions: Vec<Question> = from_str(&questions_json)?;
    Ok(questions)    
}