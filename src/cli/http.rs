use std::path::PathBuf;

use clap::Parser;

use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, default_value_t = 8080, help = "Port to listen on")]
    pub port: u16,
    #[arg(short, long, help = "Path to serve", default_value = ".", value_parser = verify_path)]
    pub dir: PathBuf,
}
