use std::fs::{self, DirEntry, ReadDir};
use std::path::PathBuf;

enum TeraObject {
    QuizFile(PathBuf),
    Dir(PathBuf),
}

pub fn tera_directory() -> PathBuf {
    // TODO: Add in a way to check if this is running as a test. Then provide an alternate
    // home directory based on an env variable.
    let user_home = home::home_dir().expect("Cannot determine users home directory! Aborting.");

    user_home.join(".tera")
}

pub fn traverse_sets_directory() -> Vec<Result<PathBuf, String>> {
    let tera_directory = tera_directory();
    let sets_root = tera_directory.join("sets");

    let mut to_explore: Vec<PathBuf> = Vec::new();
    let mut question_sets: Vec<Result<PathBuf, String>> = Vec::new();

    to_explore.push(sets_root);

    while !to_explore.is_empty() {
        let current_directory = to_explore.pop().unwrap();

        for tera_object in read_dir(&current_directory) {
            match tera_object {
                Ok(TeraObject::QuizFile(path)) => question_sets.push(Ok(path)),
                Ok(TeraObject::Dir(path)) => to_explore.push(path),
                Err(msg) => question_sets.push(Err(msg)),
            }
        }
    }

    question_sets
}

fn read_dir(d: &PathBuf) -> Vec<Result<TeraObject, String>> {
    let mut contents = Vec::new();

    if let Ok(dir_contents) = fs::read_dir(d) {
        let valid_entries = filter_failed_reads(dir_contents);

        for entry in valid_entries {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    contents.push(Ok(TeraObject::Dir(entry.path())));
                } else if meta.is_file() && named_as_yaml(&entry) {
                    contents.push(Ok(TeraObject::QuizFile(entry.path())));
                } else {
                    let err_message = format!("Unknown file type for: {}", entry.path().display());
                    contents.push(Err(err_message));
                }
            } else {
                contents.push(Err(format!(
                    "Can't read metadata for: {}",
                    entry.path().display()
                )));
            }
        }
    } else {
        contents.push(Err(format!(
            "Can't read directory contents for {}",
            d.display()
        )));
    }
    contents
}

fn filter_failed_reads(dir: ReadDir) -> impl Iterator<Item = DirEntry> {
    dir.inspect(|entry| {
        if entry.is_err() {
            println!(
                "Error while reading file/directory: {}",
                entry.as_ref().unwrap_err()
            );
        }
    })
    .flatten()
}

fn named_as_yaml(f: &DirEntry) -> bool {
    f.file_name().to_str().map_or(false, |name| {
        name.ends_with(".yml") || name.ends_with(".yaml")
    })
}
