use clap::Parser;

macro_rules! days {
    { $( $name:ident )* } => {
        fn run_day(d: usize, p: usize) {
            match format!("day{}", d).as_str() {
                $(
                    stringify!($name) => $name::main(p),
                )*
                _ => eprintln!("no such day: {} part: {}", d, p),
            }
        }
    }
}


mod util;
mod day1;
mod day2;
mod day3;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: usize,

    #[arg(short, long)]
    part: usize,
}

days!{ day1 day2 day3 }

fn main() {
    let args = Args::parse();

    run_day(args.day, args.part);
}
