        use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "100")]
    pub batch_size: usize,
    #[arg(short, long)]
    pub program_id: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
