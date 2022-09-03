use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Quiz {
    questions: Vec<Question>
}

#[derive(Deserialize, Debug)]
pub enum Question {
    TF {
        q: String,
        a: bool
    },
    Multi {
        q: String,
        options: Vec<String>,
        a: String
    },
    Response {
        q: String,
        a: String
    }
}

pub fn read_quiz<P: AsRef<Path>>(path_to_quiz: P) -> Result<Quiz, String> {
    Err("unimplemented".to_string())
}

