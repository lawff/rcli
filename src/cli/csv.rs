use clap::{Parser, ValueEnum};
use core::fmt;

use crate::CmdExector;

use super::verify_input_file;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path", value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, help = "Output format", value_enum, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',', help = "Delimiter character")]
    pub delimiter: char,
    #[arg(long, default_value_t = true, help = "Include header")]
    pub header: bool,
}

impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output
        } else {
            format!("output.{}", self.format)
        };
        crate::process_csv(&self.input, output, self.format)
    }
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
