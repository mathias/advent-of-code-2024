use clap::Parser;

mod util;
mod day1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: usize,

    #[arg(short, long)]
    part: usize,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1::main(args.part),
        _ => panic!("Unknown day!")
    }
}
