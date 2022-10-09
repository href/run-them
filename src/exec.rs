use std::ffi::OsStr;
use std::io::{BufRead, BufReader, Result};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::Mutex;
use std::thread;

static PRINT_LOCK: Mutex<Vec<u8>> = Mutex::new(Vec::new());

pub fn run<T: AsRef<OsStr>>(args: Vec<T>) -> Result<ExitStatus> {
    let (cmd, rest) = args.split_at(1);

    let mut child = Command::new(&cmd[0])
        .args(rest)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Okay to unwrap, since we set stdout/stderr above.
    let stdout = BufReader::new(child.stdout.as_mut().unwrap()).lines();
    let stderr = BufReader::new(child.stderr.as_mut().unwrap()).lines();

    // Print stdout/stderr independently
    thread::scope(|s| {
        s.spawn(|| {
            for result in stdout {
                match result {
                    Ok(line) => {
                        let lock = PRINT_LOCK.lock().unwrap();
                        println!("{}", line);
                        std::mem::drop(lock);
                    }
                    Err(e) => {
                        let lock = PRINT_LOCK.lock().unwrap();
                        eprintln!("{}", e);
                        std::mem::drop(lock);
                    }
                }
            }
        });

        s.spawn(|| {
            for result in stderr {
                match result {
                    Ok(line) => {
                        let lock = PRINT_LOCK.lock().unwrap();
                        eprintln!("{}", line);
                        std::mem::drop(lock);
                    }
                    Err(e) => {
                        let lock = PRINT_LOCK.lock().unwrap();
                        eprintln!("{}", e);
                        std::mem::drop(lock);
                    }
                }
            }
        });
    });

    child.wait()
}
