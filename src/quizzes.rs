use crate::files::traverse_sets_directory;
use serde::Deserialize;
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
    NotFound,
    Ambiguous(Vec<PathBuf>),
}

pub fn read_quiz<P: AsRef<Path>>(path_to_quiz: P) -> Result<Quiz, String> {
    if let Ok(quiz_contents) = fs::read_to_string(path_to_quiz) {
        return match serde_yaml::from_str::<Quiz>(&quiz_contents) {
            Ok(quiz) => Ok(quiz),
            Err(e) => {
                println!("{:?}", e);
                return Err("Failed to parse YAML".to_string());
            }
        }
    }
    Err("Failed to read file".to_string())
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
        return Err(QuizLookupError::NotFound);
    } else if matches.len() > 1 {
        return Err(QuizLookupError::Ambiguous(matches));
    }

    Ok(matches
        .pop()
        .expect("`matches` should have exactly 1 element"))
}
