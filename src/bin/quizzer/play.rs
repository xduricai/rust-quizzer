use std::io;
use anyhow::bail;
use quizzer::load::load_questions;
use quizzer::question::*;
use quizzer::error::Error;
use crate::validators;

pub fn play_mode() -> anyhow::Result<()> {
    let questions: Vec<Question> = load_questions()?;
    if questions.is_empty() {
        bail!(Error::NoQuestionsError)
    }
    let mut correct: usize = 0;

    for question in &questions {
        println!("{}", question);

        println!("Enter the number of the correct answer: ");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let mut index: usize = buffer.trim().parse::<usize>()?;
        index = validators::validate_index(index)?;

        if question.answers[index].correct {
            println!("Correct answer!\n");
            correct += 1;
        }
        else {
            println!("Incorrect answer!\n");
        }
    }

    println!("You answered {} out of {} questions correctly!", correct, questions.len());

    Ok(())
}