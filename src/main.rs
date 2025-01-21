use std::io;
use std::process::ExitCode;

fn sub() -> Result<(), io::Error> {
    rs_find_empty_line::find::args2filtered_names2stdout()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
