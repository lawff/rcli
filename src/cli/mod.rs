mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use std::path::{Path, PathBuf};

use clap::{command, Parser, Subcommand};
use enum_dispatch::enum_dispatch;

pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};

#[derive(Debug, Parser)]
#[command(version, about, author, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File not found: {}", filename))
    }
}

fn verify_path(path: &str) -> Result<PathBuf, String> {
    let p = Path::new(path);
    if Path::new(path).exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err(format!("Path not found: {}", path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_input_file("*"),
            Err(format!("File not found: {}", "*"))
        );
        assert_eq!(
            verify_input_file("not-exist"),
            Err(format!("File not found: {}", "not-exist"))
        );
    }
}
