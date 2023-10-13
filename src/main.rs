mod args;
mod compiler;
#[cfg(test)]
mod tests;
mod utils;

use crate::{
    compiler::compile_to_file,
    utils::{cli_info, DEBUG},
};

use std::{
    env::var,
    error::Error,
    fs::remove_file,
    process::{Command, Stdio},
    time::Instant,
};

fn main() -> Result<(), Box<dyn Error>> {
    *DEBUG.lock().unwrap() = var("DEBUG").is_ok();

    let args = args::get_args();

    if let Some(file) = args.compile {
        cli_info(&format!(
            "Compiling {file}{}",
            if args.release { " in release mode" } else { "" }
        ));

        let start = Instant::now();
        compile_to_file(file, args.out, args.release);
        let end = start.elapsed();

        cli_info(&format!("Compiliation finished in {}s", end.as_secs_f64()));
    } else if let Some(file) = args.run {
        cli_info(&format!(
            "Compiling and running {file}{}",
            if args.release { " in release mode" } else { "" }
        ));

        let start = Instant::now();
        compile_to_file(file, args.out.clone(), args.release);
        let end = start.elapsed();

        cli_info(&format!("Compiliation finished in {}s", end.as_secs_f64()));
        cli_info("Running the compiled binary");

        Command::new(format!("./{}", args.out))
            .stdout(Stdio::inherit())
            .status()
            .expect("Failed to run compiled binary");

        remove_file(args.out).expect("Failed to remove compiled binary");
    } else {
        eprintln!("Incorrect usage. Check --help for more information.");
    }

    Ok(())
}
