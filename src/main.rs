use clap::{Parser, Subcommand};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

#[derive(Parser)]
#[command(name = "datagen", about = "Generate fake data files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Csv {
        #[arg(default_value = "output.csv")]
        path: String,

        #[arg(short, long, default_value_t = 10)]
        lines: u32,
    },

    Xml {
        #[arg(default_value = "output.xml")]
        path: String,

        #[arg(short, long, default_value_t = 10)]
        lines: u32,
    },
}

fn open_file(path: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
}

fn write_csv(path: &str, lines: u32) -> Result<(), io::Error> {
    let mut file = open_file(&path)?;

    let delim = ",";

    // Write Headers
    write!(file, "index{delim}data\n")?;

    // Write Data
    for i in 1..=lines {
        write!(file, "{i}{delim}data\n")?;
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Csv { path, lines } => match write_csv(&path, lines) {
            Ok(_file) => println!("Opened successfully"),
            Err(e) => println!("Failed: {e}"),
        },
        Commands::Xml { path, lines: _ } => match open_file(&path) {
            Ok(_file) => println!("Opened successfully"),
            Err(e) => println!("Failed: {e}"),
        },
    }
}
