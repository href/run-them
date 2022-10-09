mod exec;

use clap::Parser;
use exec::run;
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "1")]
    worker: usize,

    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let success = Arc::new(AtomicBool::new(true));

    thread::scope(|scope| {
        for worker in 0..args.worker {
            let command = args.command.clone();
            let thread_success = success.clone();

            scope.spawn(move || match run(command) {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("warning: worker {worker} exited with {status}");
                        thread_success.store(false, Ordering::Relaxed);
                    }
                }
                Err(e) => {
                    eprintln!("error: worker {worker} could not start: {e}");
                    thread_success.store(false, Ordering::Relaxed);
                }
            });
        }
    });

    match success.load(Ordering::Relaxed) {
        true => ExitCode::SUCCESS,
        false => ExitCode::from(1),
    }
}
