pico_args_helpgen::define_app! {
    app_name: "Pro-Lang",
    app_description: "Programming language for PROs",
    app_version: "v0.1.0",

    help_args: "-h, --help",
    version_args: "-V, --version",

    struct AppArgs {
        compile: Option<String>, "-c, --compile", "[file] Compile given file",
        run: Option<String>, "-r, --run", "[file] Compile and run given file",
        out: String, "-o, --out", "[file] Output file",
        release: bool, "--release", "Enable release mode builds (using GCC)",
        force: bool, "--force", "Force recompilation even if change was not detected",
        test: bool, "-t, --test", "Run tests from tests.pro file",
    }
}

fn parse_args() -> Result<AppArgs, pico_args_helpgen::Error> {
    let mut pargs = pico_args_helpgen::Arguments::from_env();

    handle_help_version();

    let args = AppArgs {
        compile: pargs.value_from_str(["-c", "--compile"]).ok(),
        run: pargs.value_from_str(["-r", "--run"]).ok(),
        out: pargs
            .value_from_str(["-o", "--out"])
            .unwrap_or("out.app".to_string()),
        release: pargs.contains("--release"),
        force: pargs.contains("--force"),
        test: pargs.contains(["-t", "--test"]),
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Unexpected argument(s): {:?}", remaining);
        std::process::exit(1);
    }

    Ok(args)
}

pub fn get_args() -> AppArgs {
    parse_args().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    })
}
