fn compile string input_file {
    () println "TODO compiler"
    () println %s input_file

    var VecString stringvec () newVecString

    () pushVecString &stringvec "test1"
    () pushVecString &stringvec "test2"
    () pushVecString &stringvec "test3"

    var string s1 () getVecString &stringvec 0
    () println %s s1

    var int i 0
    loop i < stringvec.size {
        var string s () getVecString &stringvec i
        () println %s s
        ++ i
    }

    var VecString compiler_lines () readToLines "main.pro"
    var int x 0
    loop x < compiler_lines.size {
        var string s () getVecString &compiler_lines x
        = s () stringTrim s
        var bool empty () stringEmpty s
        if !empty {
            () println %s s
        }
        ++ x
    }

    var VecString compiler_lines_2 () readToLinesNonEmpty "main.pro"
    = x 0
    loop x < compiler_lines_2.size {
        var string s () getVecString &compiler_lines_2 x
        = s () stringTrim s
        () println %s s
        ++ x
    }
}
