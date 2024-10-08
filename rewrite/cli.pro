var string app_name "ProLang"
var string app_version "v0.1.0"
var string output_file "out.app"
var bool release_mode false
var bool force_recompilation false

import compiler.pro
import utils.pro

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
    () println "\t--force\t\t\t- Force recompilation even if change was not detected"
    () println
    () println "\t-h, --help\t\t- Display this help screen"
    () println "\t-V, --version\t\t- Display version information"
}

fn handle_args int argc char* argv[] > int {
    var VecString args () getArgsVec argc argv

    var bool ddhelp () containsVecString &args "--help"
    var bool dh () containsVecString &args "-h"
    if argc == 1 || ddhelp || dh {
        () print_help
        ret 0
    }

    var bool ddversion () containsVecString &args "--version"
    var bool dV () containsVecString &args "-V"
    if ddversion || dV {
        () print_version
        ret 0
    }

    var bool ddout () containsVecString &args "--out"
    var bool dout () containsVecString &args "-o"
    if ddout || dout {
        if argc < 3 {
            () eprintln "ERROR: output file not provided"
            ret 1
        }
        if ddout {
            var int x () findVecString &args "--out"
            = output_file () getVecString &args x+1
        }
        if dout {
            var int x () findVecString &args "-o"
            = output_file () getVecString &args x+1
        }
    }

    var bool ddrelease () containsVecString &args "--release"
    if ddrelease {
        = release_mode true
    }

    var bool ddforce () containsVecString &args "--force"
    if ddforce {
        = force_recompilation true
    }

    var bool ddcompile () containsVecString &args "--compile"
    var bool dc () containsVecString &args "-c"
    if ddcompile || dc {
        var string input_file ""
        if ddcompile {
            var int x () findVecString &args "--compile"
            if argc <= x+1 {
                () eprintln "ERROR: input file not provided"
                ret 1
            }
            = input_file () getVecString &args x+1
        }
        if dc {
            var int x () findVecString &args "-c"
            if argc <= x+1 {
                () eprintln "ERROR: input file not provided"
                ret 1
            }
            = input_file () getVecString &args x+1
        }
        () compile input_file

        ret 0
    }

    var bool ddrun () containsVecString &args "--run"
    var bool dr () containsVecString &args "-r"
    if ddrun || dr {
        var string input_file ""
        if ddrun {
            var int x () findVecString &args "--run"
            if argc <= x+1 {
                () eprintln "ERROR: input file not provided"
                ret 1
            }
            = input_file () getVecString &args x+1
        }
        if dr {
            var int x () findVecString &args "-r"
            if argc <= x+1 {
                () eprintln "ERROR: input file not provided"
                ret 1
            }
            = input_file () getVecString &args x+1
        }
        () compile input_file
        () run output_file

        ret 0
    }

    () eprintln "ERROR: incorrect usage"
    () eprintln "Check the help page using --help"
    ret 1
}
