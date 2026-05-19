use clap::{Parser, Subcommand};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Csv {
        path: String,

        #[arg(short, long, default_value_t = 10)]
        lines: u32,
    },

    Xml {
        path: String,

        #[arg(short, long, default_value_t = 10)]
        lines: u32,
    },
}

fn open_file(path: &str) -> Result<File, io::Error> {
    let res = OpenOptions::new().append(true).create(true).open(path)?;
    Ok(res)
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Csv { path, lines: _ } => match open_file(&path) {
            Ok(_file) => println!("Opened successfully"),
            Err(e) => println!("Failed: {e}"),
        },
        Commands::Xml { path, lines: _ } => match open_file(&path) {
            Ok(_file) => println!("Opened successfully"),
            Err(e) => println!("Failed: {e}"),
        },
    }
}
