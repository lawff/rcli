use clap::Parser;
use enum_dispatch::enum_dispatch;
use jsonwebtoken::Algorithm;

use crate::{process_jwt_sign, process_jwt_verify, CmdExector};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "Sign a JWT token")]
    Sign(SignOpts),
    #[command(name = "verify", about = "Verify a JWT token")]
    Verify(VerifyOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(long, help = "JWT data")]
    pub sub: String,
    #[arg(long, help = "JWT algorithm", default_value = "HS256")]
    pub alg: Algorithm,
    #[arg(long, help = "JWT expiration time", default_value = "1d", value_parser = parse_duration)]
    pub exp: usize,
    #[arg(long, help = "JWT audience", default_value = "rcli")]
    pub aud: String,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short, long, help = "JWT token")]
    pub token: String,
    #[arg(long, help = "JWT algorithm", default_value = "HS256")]
    pub alg: Algorithm,
    #[arg(long, help = "JWT audience", default_value = "rcli")]
    pub aud: String,
}

fn parse_duration(s: &str) -> Result<usize, String> {
    let len = s.len();
    let mut num = s;
    let mut unit = "s"; // 默认单位为秒

    if s.ends_with('s') || s.ends_with('m') || s.ends_with('h') || s.ends_with('d') {
        let (num_part, unit_suffix) = s.split_at(len - 1);
        num = num_part;
        unit = unit_suffix;
    }

    let num = match num.parse::<usize>() {
        Ok(n) => n,
        Err(e) => return Err(format!("Invalid duration number: {}", e)),
    };
    match unit {
        "s" => Ok(num),
        "m" => Ok(num * 60),
        "h" => Ok(num * 60 * 60),
        "d" => Ok(num * 60 * 60 * 24),
        _ => Err(format!("Invalid duration unit: {}", unit)),
    }
}

impl CmdExector for SignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = process_jwt_sign(self.sub, self.exp, self.aud, self.alg)?;
        println!("{}", token);

        Ok(())
    }
}

impl CmdExector for VerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        match process_jwt_verify(self.token, self.aud, self.alg) {
            Ok(true) => println!("JWT token is valid"),
            Ok(false) => println!("JWT token is invalid"),
            Err(e) => eprintln!("JWT token is invalid, Error: {}", e),
        }

        Ok(())
    }
}
