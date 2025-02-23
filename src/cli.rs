use clap::Parser;

#[derive(Debug, Parser)]
// #[command(version, about, long_about=None)]  // Version read from Cargo.toml
#[command(version, about)] // Version read from Cargo.toml
pub struct Args {
    #[arg(short, long)]
    pub output: Option<std::path::PathBuf>,

    #[arg(short, long)]
    pub input: Option<std::path::PathBuf>,

    #[arg(short, long)]
    pub verbose: bool,
}
