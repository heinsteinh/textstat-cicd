//! `textstat` command-line entry point.
//!
//! Usage: `textstat [FILE]` — reads FILE, or stdin when FILE is omitted.

use std::fs::File;
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let result = match args.as_slice() {
        [] => textstat::count_reader(io::stdin().lock()),
        [path] => File::open(path).and_then(textstat::count_reader),
        _ => {
            eprintln!("usage: textstat [FILE]");
            return ExitCode::from(2);
        }
    };

    match result {
        Ok(stats) => {
            println!("{} {} {}", stats.lines, stats.words, stats.bytes);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("textstat: {err}");
            ExitCode::FAILURE
        }
    }
}
