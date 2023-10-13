var char* app_name "CLI1"
var char* app_version "v0.1.0"

fn print_version {
    () print %s app_name
    () print "_"
    () println %s app_version
}

fn print_help {
    () print %s app_name
    () println " - Basic_CLI_app"
    () print "Version: "
    () println %s app_version
    () println
    () println "Usage: cli1 [options]"
    () println
    () println "Options:"
    () println "____-h, --help___ - Display_this_help_screen"
    () println "____-v, --version - Display_version_information"
}

fn main int argc char* argv[] > int {
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
