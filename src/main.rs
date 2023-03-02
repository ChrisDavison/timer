use std::io::{self, Write};
use structopt::StructOpt;

mod timer;

#[derive(StructOpt, Debug)]
#[structopt(name = "timer")]
struct Opts {
    #[structopt(short="f", long="file")]
    file: Option<String>,
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
    let reset = format!("\r{}\r", " ".repeat(format!("{}", timer).len()));
    let mut stdout = io::stdout();
    while !timer.finished() {
        match &args.file {
            Some(fname) => {
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(fname)?;
                file.write_all(format!("{timer}").as_bytes())?;
            },
            _ => {
                stdout.write_all(format!("{reset}{timer}").as_bytes())?;
                stdout.flush()?;
            }
        }
        timer.tick();
    }
    if let Some(fname) = args.file {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(fname)?;
        file.write_all(format!("Done.\n").as_bytes())?;
    }

    println!("{}DONE.", reset);
    Ok(())
}
