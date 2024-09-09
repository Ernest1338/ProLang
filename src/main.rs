mod args;
mod compiler;
#[cfg(test)]
mod tests;
mod utils;

use crate::{
    args::AppArgs,
    compiler::compile_to_file,
    utils::{choose_compiler, cli_info, DEBUG},
};

use std::{
    env::var,
    error::Error,
    fs::remove_file,
    process::{Command, Stdio},
    time::Instant,
};

fn compile_and_run(file: &str, args: &AppArgs) {
    cli_info(&format!(
        "Compiling and running {file}{} using {}",
        if args.release { " in release mode" } else { "" },
        choose_compiler()
    ));

    let start = Instant::now();
    compile_to_file(file.to_string(), args.out.clone(), args.release, args.force);
    let end = start.elapsed();

    cli_info(&format!(
        "Compiliation finished in {:.3}s",
        end.as_secs_f64()
    ));
    cli_info("Running the compiled binary");

    Command::new(format!("./{}", args.out))
        .stdout(Stdio::inherit())
        .status()
        .expect("Failed to run compiled binary");

    remove_file(args.out.clone()).expect("Failed to remove compiled binary");
}

fn compile(file: &str, args: &AppArgs) {
    cli_info(&format!(
        "Compiling {file}{} using {}",
        if args.release { " in release mode" } else { "" },
        choose_compiler()
    ));

    let start = Instant::now();
    compile_to_file(file.to_string(), args.out.clone(), args.release, args.force);
    let end = start.elapsed();

    cli_info(&format!(
        "Compiliation finished in {:.3}s",
        end.as_secs_f64()
    ));
}

fn main() -> Result<(), Box<dyn Error>> {
    *DEBUG.lock().unwrap() = var("DEBUG").is_ok();

    let args = args::get_args();

    if args.test {
        compile_and_run("tests.pro", &args);
    } else if let Some(ref file) = args.compile {
        compile(&file, &args);
    } else if let Some(ref file) = args.run {
        compile_and_run(&file, &args);
    } else {
        eprintln!("Incorrect usage. Check --help for more information.");
    }

    Ok(())
}
