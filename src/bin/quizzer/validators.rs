use quizzer::error::Error;

pub fn validate_input(input: &str) -> anyhow::Result<String, Error> {
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() { Err(Error::EmptyInput) } else { Ok(trimmed) }
}

pub fn validate_index(index: usize) -> anyhow::Result<usize, Error> {
    if !(1..=4).contains(&index) { Err(Error::InvalidAnswerNumber) } else { Ok(index - 1) }
}