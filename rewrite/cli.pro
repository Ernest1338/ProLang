var char* app_name "ProLang"
var char* app_version "v0.1.0"

fn print_version {
    () print %s app_name
    () print " "
    () println %s app_version
}

fn print_help {
    () print %s app_name
    () println " - Programming language for PROs"
    () print "Version: "
    () println %s app_version
    () println
    () println "Usage: prolang [options]"
    () println
    () println "Options:"
    () println "\t-c, --compile\t [file] - Compile given file"
    () println "\t-r, --run\t [file] - Compile and run given file"
    () println "\t-o, --out\t [file] - Output file"
    () println "\t--release\t\t- Enable release mode builds (using GCC)"
    () println
    () println "\t-h, --help\t\t- Display this help screen"
    () println "\t-V, --version\t\t- Display version information"
}

fn handle_args int argc char* argv[] {
    if argc == 1 || strcmp(argv[1], "--help") == 0 || strcmp(argv[1], "-h") == 0 {
        () print_help
    }
    elif strcmp(argv[1], "-V") == 0 || strcmp(argv[1], "--version") == 0 {
        () print_version
    }
    else {
        () println "Incorrect usage. Check --help for more information"
    }
}
