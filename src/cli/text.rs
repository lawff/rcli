use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use super::{verify_input_file, verify_path};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(
        name = "sign",
        about = "Sign a text with a private/session key and output the signature"
    )]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "Verify a text with a private/session key")]
    Verify(TextVerifyOpts),
    #[command(
        name = "generate",
        about = "Generate a random blake3 key or ed25519 key pair"
    )]
    Generate(KeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, value_parser = verify_input_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, value_parser = verify_input_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Copy, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid text sign format: {}", s)),
        }
    }
}
