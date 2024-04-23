use crate::CmdExector;
use clap::{ArgAction, Parser};
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16, help = "Password length")]
    pub length: u8,

    #[arg(
        long,
        action = ArgAction::Set,
        default_value_t = true,
        help = "Include uppercase letters"
    )]
    pub uppercase: bool,

    #[arg(
        long,
        action = ArgAction::Set,
        default_value_t = true,
        help = "Include lowercase letters"
    )]
    pub lowercase: bool,

    #[arg(long, action = ArgAction::Set, default_value_t = true, help = "Include numbers")]
    pub number: bool,

    #[arg(long, action = ArgAction::Set, default_value_t = true, help = "Include symbols")]
    pub symbol: bool,
}

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        println!("{}", ret);

        // output password strength in stderr
        let estimate = zxcvbn(&ret, &[])?;
        eprintln!("Password strength: {}", estimate.score());
        Ok(())
    }
}
