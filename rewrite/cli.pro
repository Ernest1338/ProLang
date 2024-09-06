import compiler.pro

var string app_name "ProLang"
var string app_version "v0.1.0"
var string out_filename "out.app"

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

fn handle_args int argc char* argv[] > int {
    // FIXME: not if argv[1] is something but if argv CONTAINS something

    if argc == 1 || strcmp(argv[1], "--help") == 0 || strcmp(argv[1], "-h") == 0 {
        () print_help
        ret 0
    }

    if strcmp(argv[1], "-V") == 0 || strcmp(argv[1], "--version") == 0 {
        () print_version
        ret 0
    }

    // if strcmp(argv[1], "-o") == 0 || strcmp(argv[1], "--out") == 0 {
    //     if argc < 3 {
    //         () eprintln "ERROR: output file not provided"
    //         ret 1
    //     }
    //     = out_filename argv[2]
    // }

    if strcmp(argv[1], "-c") == 0 || strcmp(argv[1], "--compile") == 0  {
        if argc < 3 {
            () eprintln "ERROR: input file not provided"
            ret 1
        }

        var string input_file argv[2]
        () compile input_file

        ret 0
    }

    ret 0
}
