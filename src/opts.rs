use clap::{arg, Parser, Subcommand};
use std::path::Path;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path", value_parser = verify_file_exists)]
    pub input: String,
    #[arg(short, long, help = "Output file path", default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',', help = "Delimiter character")]
    pub delimiter: char,
    #[arg(long, default_value_t = true, help = "Include header")]
    pub header: bool,
}

fn verify_file_exists(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File not found: {}", filename))
    }
}
