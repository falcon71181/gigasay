use clap::Parser;
use gigasay::{print_message, Options};
use std::usize;
use textwrap::wrap;
use unicode_width::UnicodeWidthStr;

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

    println!("{:?}", args.message);
    println!("{:?}", args.width);

    let message = "fnonfi34nfi n34oif34 nf3i4fnssss";
    let mut lines = wrap(message, 100 as usize);
    let mut longest = lines.iter().map(|line| line.width_cjk()).max().unwrap();

    print_message(message, opts);
    Ok(())
}
