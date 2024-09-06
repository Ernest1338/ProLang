import handle_imports.pro

fn compile string input_file {
    var string source_code () readToString input_file

    = source_code () handle_imports source_code

    var VecString code_lines () splitToLines source_code

    () println "source_code:"
    var int i 0
    loop i < code_lines.size {
        var string line () getVecString &code_lines i
        () println %s line
        ++ i
    }
}
