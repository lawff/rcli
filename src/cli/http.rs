use std::path::PathBuf;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_http_serve, CmdExector};

use super::verify_path;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
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

impl CmdExector for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await
    }
}
