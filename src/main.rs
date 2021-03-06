use std::io::{self, Write};
use structopt::StructOpt;

mod timer;

#[derive(StructOpt, Debug)]
#[structopt(name = "timer")]
struct Opts {
    message: Vec<String>,
    #[structopt(short = "h", long, default_value = "0")]
    hours: u64,
    #[structopt(short = "m", long, default_value = "50")]
    minutes: u64,
    #[structopt(short = "s", long, default_value = "0")]
    seconds: u64,
}

fn main() -> Result<(), std::io::Error> {
    let args = Opts::from_args();
    let mut timer = timer::Countdown::new(
        args.hours,
        args.minutes,
        args.seconds,
        if args.message.is_empty() {
            None
        } else {
            Some(args.message.join(" "))
        },
    );
    let reset = format!("\r{}\r", " ".repeat(80));
    let mut stdout = io::stdout();
    while !timer.finished() {
        print!("{}{}", reset, timer);
        stdout.flush()?;
        timer.tick();
    }
    println!("{}DONE.", reset);
    Ok(())
}
