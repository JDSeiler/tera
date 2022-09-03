use clap::{Parser, Subcommand};
use std::fs::{self, DirEntry, ReadDir};
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize Tera
    Init {
        /// Overwrite the Tera directory, if it already exists
        #[clap(short, long, action)]
        force: bool,
    },
    /// List all available quizzes
    List,
    /// Take a quiz
    Take {
        /// The name of the quiz to take
        #[clap(value_parser)]
        quiz: String,
    },
}

enum TeraObject {
    QuizFile(PathBuf),
    Dir(PathBuf),
    Error(String),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Command::Init { force } => {
            println!("Initializing Tera. Will we force? {}", force);
            initialize(*force).map_or_else(
                |e| println!("Could not initialize: {:?}", e.to_string()),
                |_| println!("The ~/.tera directory has been succesfully initialized"),
            )
        }
        Command::List => {
            list()
        }
        Command::Take { quiz } => {
            println!("Taking quiz: {}", quiz)
        }
    }
}

fn list() {
    let tera_directory = tera_directory();
    let sets_root = tera_directory.join("sets");

    let mut to_explore: Vec<PathBuf> = Vec::new();
    let mut question_sets: Vec<PathBuf> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    to_explore.push(sets_root);

    while !to_explore.is_empty() {
        let current_directory = to_explore.pop().unwrap();

        for tera_object in read_dir(&current_directory) {
            match tera_object {
                TeraObject::QuizFile(path) => question_sets.push(path),
                TeraObject::Dir(path) => to_explore.push(path),
                TeraObject::Error(msg) => errors.push(msg),
            }
        }
    }

    pretty_print(&question_sets);
}

fn read_dir(d: &PathBuf) -> Vec<TeraObject> {
    let mut contents = Vec::new();

    if let Ok(dir_contents) = fs::read_dir(d) {
        let valid_entries = filter_failed_reads(dir_contents);

        for entry in valid_entries {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    contents.push(TeraObject::Dir(entry.path()));
                } else if meta.is_file() && named_as_yaml(&entry) {
                    contents.push(TeraObject::QuizFile(entry.path()));
                } else {
                    let err_message = format!("Unknown file type for: {}", entry.path().display());
                    contents.push(TeraObject::Error(err_message));
                }
            }
        }
    } else {
        contents.push(TeraObject::Error(format!(
            "Can't read directory contents for {}",
            d.display()
        )));
    }
    contents
}

fn pretty_print(paths: &[PathBuf]) {
    let tera_directory = tera_directory();
    let sets_root = tera_directory.join("sets");

    // Result implements IntoIterator in such a way that you can
    // treat: Vec<Result<T, E>>
    // like: Vec<Vec<T>>, where all E values are discarded.
    // So, you can use flatten or flat_map to extract the
    // inner T values and ignore the E values.
    let mut relative_paths: Vec<&Path> = paths
        .iter()
        .flat_map(|p| p.strip_prefix(&sets_root))
        .collect();

    // Tuples with 0 to 12 fields implement Ord, so you can use them
    // to sort by multiple values at once. Here we first sort by path length,
    // Then we sort lexicographically between paths of the same length.
    relative_paths.sort_by_key(|p| (path_length(p), p.to_str().unwrap_or("")));

    relative_paths
        .iter()
        .filter_map(|p| p.to_str())
        .for_each(|p| println!("{}", p));
}

fn path_length(p: &Path) -> usize {
    p.iter().fold(0, |accum, _item| accum + 1)
}

fn named_as_yaml(f: &DirEntry) -> bool {
    f.file_name().to_str().map_or(false, |name| {
        name.ends_with(".yml") || name.ends_with(".yaml")
    })
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

fn tera_directory() -> PathBuf {
    let user_home = home::home_dir().expect("Cannot determine users home directory! Aborting.");

    user_home.join(".tera")
}

fn initialize(force: bool) -> io::Result<()> {
    let tera_directory = tera_directory();
    let tera_sets = tera_directory.join("sets");
    // TODO: This isn't all that platform agnostic, I don't think.
    // TODO: Consider what would happen if .tera was a file.
    if !tera_directory.is_dir() {
        println!("~/.tera does not exist or is not a directory, creating folders...");
        fs::create_dir_all(&tera_directory).and_then(|()| fs::create_dir(&tera_sets))
    } else {
        if force {
            println!("Forcefully creating ~/.tera folder");
            // GULP, big danger doing recursive delete!
            return fs::remove_dir_all(&tera_directory)
                .and_then(|()| fs::create_dir_all(&tera_directory))
                .and_then(|()| fs::create_dir(&tera_sets));
        }
        Err(Error::new(
            ErrorKind::AlreadyExists,
            "~/.tera already exists and the --force flag was not specified!",
        ))
    }
}
