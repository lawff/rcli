use clap::Parser;
use enum_dispatch::enum_dispatch;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{get_content, CmdExector};

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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    exp: usize,
}

fn parse_duration(s: &str) -> Result<usize, String> {
    let len = s.len();
    let (num, unit) = s.split_at(len - 1);

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
        let now = chrono::Utc::now().timestamp() as usize;
        let my_claims = Claims {
            sub: self.sub,
            exp: now + self.exp,
            aud: self.aud,
        };
        let key = get_content("fixtures/jwt_secret.key")?;
        let key = key.as_slice();

        let header = Header {
            alg: self.alg,
            ..Default::default()
        };

        let token = encode(&header, &my_claims, &EncodingKey::from_secret(key))?;
        println!("{}", token);

        Ok(())
    }
}

impl CmdExector for VerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = self.token;
        let key = get_content("fixtures/jwt_secret.key")?;
        let key = key.as_slice();

        let mut validation = Validation::new(self.alg);
        validation.set_audience(&[self.aud]);
        validation.set_required_spec_claims(&["aud"]);

        let decoded = decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation)?;

        println!("{:?}", decoded.claims);
        println!("{:?}", decoded.header);

        Ok(())
    }
}
