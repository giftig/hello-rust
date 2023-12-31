use std::option::Option;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum DemoType {
    All,
    Async,
    Books,
    Channels,
    CrabSay,
    Json,
    Ownership,
    Woof
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(super) struct Args {
    #[clap(value_enum, short, long, default_value_t=DemoType::All)]
    pub demo: DemoType,

    #[arg(short, long, default_value_t=String::from("I'm a stupid shit-eating crab"))]
    pub crab_phrase: String
}
