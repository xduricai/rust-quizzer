use std::io;
use quizzer::load::load_questions;
use quizzer::save::save_questions;
use quizzer::question::*;

use crate::validators;

pub fn input_mode() -> anyhow::Result<()> {
    let mut questions: Vec<Question> = load_questions()?;

    loop {
        println!("Enter a question or type 'Exit' to exit input mode: ");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        if buffer.trim().to_lowercase().eq(&String::from("exit")) { break };

        let question_text = validators::validate_input(&buffer)?;
        
        let mut answers: [Answer; 4] = [Answer::new(), Answer::new(), Answer::new(), Answer::new()];

        for n in 0..4 {
            println!("Enter answer #{}: ", n + 1);
            buffer.clear();
            io::stdin().read_line(&mut buffer)?;
            answers[n].text = validators::validate_input(&buffer)?;
        }

        println!("Enter the number of the correct answer: ");
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        let mut index: usize = buffer.trim().parse::<usize>()?;

        index = validators::validate_index(index)?;
        answers[index].correct = true;
        
        let question = Question {
            text: question_text,
            answers
        };

        questions.push(question);
    }

    if !questions.is_empty() {
        save_questions(questions)?;
    }

    Ok(())
}