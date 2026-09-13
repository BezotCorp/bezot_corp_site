use std::ffi::OsString;
use std::io;
use std::process::Command;

pub fn run_command(program: &str, args: Vec<String>) -> io::Result<()> {
    let status = Command::new(program)
        .args(args.iter().map(OsString::from))
        .status()?;

    if status.success() {
        Ok(())
    } else {
        let command = format!("{program} {}", args.join(" "));
        let message = match status.code() {
            Some(status) => format!("{command} exited with status {status}"),
            None => format!("{command} was terminated by a signal"),
        };
        Err(io::Error::other(message))
    }
}
