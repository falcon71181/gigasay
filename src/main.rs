use clap::Parser;
use gigasay::{print_message, Options};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[clap(long, short)]
    width: Option<u32>,

    #[clap(default_value = "Go to gym my kiddo")]
    message: String,
}

fn main() -> Result<(), ()> {
    let args = Cli::parse();

    let width = args.width.unwrap_or(50);

    // INFO: additional options goes here
    let opts = Options { width };

    print_message(&args.message, opts);
    Ok(())
}
