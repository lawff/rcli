use clap::{ArgAction, Parser};

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
