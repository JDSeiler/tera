use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Error, ErrorKind};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    /// Initialize Tera
    Init {
        /// Overwrite the Tera directory, if it already exists
        #[clap(short, long, action)]
        force: bool
    },
    /// List all available quizzes
    List,
    /// Take a quiz
    Take {
        /// The name of the quiz to take
        #[clap(value_parser)]
        quiz: String
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Command::Init { force } => {
            println!("Initializing Tera. Will we force? {}", force);
            initialize(*force).map_or_else(
                |e| println!("Could not initialize: {:?}", e.to_string()),
                |_| println!("The ~/.tera directory has been succesfully initialized")
            )
        },
        Command::List => {
            println!("Listing all quizzes")
        },
        Command::Take { quiz } => {
            println!("Taking quiz: {}", quiz)
        }
    }
}


fn initialize(force: bool) -> io::Result<()> {
    let user_home = home::home_dir().expect("Cannot determine users home directory! Aborting.");

    let tera_directory = user_home.join(".tera");
    let tera_sets = tera_directory.join("sets");
    // TODO: This isn't all that platform agnostic, I don't think.
    // TODO: Consider what would happen if .tera was a file.
    if !tera_directory.is_dir() {
        println!("~/.tera does not exist or is not a directory, creating folders...");
        fs::create_dir_all(&tera_directory)
            .and_then(|()| fs::create_dir(&tera_sets))
    } else {
        if force {
            println!("Forcefully creating ~/.tera folder");
            // GULP, big danger doing recursive delete!
            return fs::remove_dir_all(&tera_directory)
                .and_then(|()| fs::create_dir_all(&tera_directory))
                .and_then(|()| fs::create_dir(&tera_sets))
        }
        Err(Error::new(ErrorKind::AlreadyExists, "~/.tera already exists and the --force flag was not specified!"))
    }
}
