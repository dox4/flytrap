use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub config: Option<String>,
}

pub fn execute() -> Cli {
    Cli::parse()
}
