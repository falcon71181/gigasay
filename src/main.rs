use clap::Parser;
use std::usize;
use textwrap::wrap;
use unicode_width::UnicodeWidthStr;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[clap(long, short)]
    width: Option<u16>,

    #[clap(default_value = "Go to gym my kiddo")]
    message: String,
}

fn main() -> Result<(), ()> {
    let args = Cli::parse();

    let width = args.width.unwrap_or(50);

    // let format_opts = FormatOptions {
    //     think: args.think,
    //     width,
    // };

    println!("{:?}", args.message);
    println!("{:?}", args.width);

    let message = "fnonfi34nfi n34oif34 nf3i4fnssss";
    let mut lines = wrap(message, 100 as usize);
    let mut longest = lines.iter().map(|line| line.width_cjk()).max().unwrap();

    Ok(())
}
