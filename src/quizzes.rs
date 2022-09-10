use crate::files::traverse_sets_directory;
use serde::Deserialize;
use core::fmt;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Quiz {
    questions: Vec<Question>,
}

#[derive(Deserialize, Debug)]
pub enum Question {
    TF {
        q: String,
        a: bool,
    },
    Multi {
        q: String,
        options: Vec<String>,
        a: String,
    },
    Response {
        q: String,
        a: String,
    },
}

#[derive(Debug)]
pub enum QuizLookupError {
    NotFound(String),
    Ambiguous(Vec<PathBuf>),
    BadYaml(String),
    IoFailure(String)
}

impl fmt::Display for QuizLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuizLookupError::NotFound(path) => write!(f, "Could not find the requested quiz: {}", path),
            QuizLookupError::Ambiguous(all_matching_files) => {
                let message = format!("Requested quiz matched multiple files. Matches:\n{:#?}", all_matching_files);
                write!(f, "{}", message)
            },
            QuizLookupError::BadYaml(error_message) => write!(f, "Failed to parse YAML! Reason: {}", error_message),
            QuizLookupError::IoFailure(error_message) => write!(f, "Failed to read quiz file from disk! Reason: {}", error_message),
        }
    }
}

pub fn read_quiz<P: AsRef<Path>>(path_to_quiz: P) -> Result<Quiz, QuizLookupError> {
    match fs::read_to_string(path_to_quiz) {
        Ok(quiz_contents) => {
            match serde_yaml::from_str::<Quiz>(&quiz_contents) {
                Ok(quiz) => Ok(quiz),
                Err(e) => {
                    Err(QuizLookupError::BadYaml(e.to_string()))
                }
            }
        }
        Err(io_error) => {
            Err(QuizLookupError::IoFailure(io_error.to_string()))
        }
    }
}

pub fn find_quiz(name_or_path: String) -> Result<PathBuf, QuizLookupError> {
    let target_path = PathBuf::from(name_or_path);

    let all_quiz_files = traverse_sets_directory();
    let mut matches: Vec<PathBuf> = all_quiz_files
        .iter()
        .flatten()
        .filter(|path| path.ends_with(&target_path))
        .cloned()
        .collect();
    if matches.is_empty() {
        return Err(QuizLookupError::NotFound(format!("{}", target_path.display())));
    } else if matches.len() > 1 {
        return Err(QuizLookupError::Ambiguous(matches));
    }

    Ok(matches
        .pop()
        .expect("`matches` should have exactly 1 element"))
}

#[derive(Debug)]
pub struct ValidationError {
    idx: usize,
    options: Vec<String>,
    incorrect_key: String
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Answer for question {} does not match any of the options", self.idx)
            .and_then(|_|  write!(f, "Options: {:#?}", self.options))
            .and_then(|_| write!(f, "Answer: {}", self.incorrect_key))
    }
}

pub fn validate_answer_key(quiz: &Quiz) -> Result<(), Vec<ValidationError>> {
    let errors: Vec<ValidationError> = quiz.questions.iter().enumerate().filter_map(|(idx, question)| {
        match question {
            Question::Multi { options, a, .. } => {
                if !options.contains(a) {
                    return Some(ValidationError {
                        idx,
                        options: options.to_vec(),
                        incorrect_key: a.to_string()
                    });
                }
                None
            },
            _ => None,
        }
    }).collect();
    if errors.is_empty() {
        return Ok(());
    }
    Err(errors)
}

