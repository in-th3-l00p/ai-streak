pub mod seeding;

use clap::{Parser};

#[derive(clap::ValueEnum, Debug, Clone, Default)]
#[clap(rename_all = "kebab_case")]
pub enum ExecutionMode {
    #[default]
    Server,
    Seeding
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // the execution mode of the binary
    #[arg(short, long, default_value_t, value_enum)]
    pub(crate) mode: ExecutionMode,
}