fn handle_imports string source_code > string {
    var VecString lines () splitToLines source_code
    var int i 0
    loop i < lines.size {
        var string line () getVecString &lines i

        var VecString splited () stringSplitWhitespace line
        if splited.size < 1 {
            ++ i
            continue
        }
        var string keyword () getVecString &splited 0
        var bool check () stringCmp keyword "import"

        if !check || splited.size < 2 {
            ++ i
            continue
        }

        var string file_name () getVecString &splited 1
        var string fcontent () readToString file_name
        = fcontent () handle_imports fcontent
        = lines.data[i] fcontent

        ++ i
    }

    var string joined () joinVecString &lines '\n'
    ret joined
}
