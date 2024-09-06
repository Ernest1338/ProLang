// Prints "yoo"
fn print_yoo {
    () println "yoo"
}

// Prints both arguments
// * (int) v1
// * (int) v2
// returns 0 if succedded
fn print_arg int v1 int v2 > int {
    () println %d v1
    () println %d v2

    ret 0
}

C int test_fn() {
C     printf("running test fn\n");
C }
export test_fn 0

fn main > int {
    var int foo 5

    () test_fn

    C int d1 = 1;
    C int d2 = 2;
    () print_arg d1 d2

    loop foo > 0 {
        () print "foo = "
        () println %d foo
        () print_yoo
        () print_arg 69 420

        -- foo
        = foo foo - 1
    }

    () print "foo = "
    () println %d foo

    if foo == 0 {
        () println "test"
    }
    elif foo < 0 {
        () println "less than zero"
    }
    else {
        () println "positive"
    }

    var int result () print_arg 10 20
    if result == 0 {
        () println %s "success"
    }

    var int bar
    = bar 2137
    () print "bar = "
    () println %d bar

    = bar () print_arg 30 40
    () print "bar = "
    () println %d bar

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

    var VecString compiler_lines () readToLines "helloworld.pro"
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

    var VecString compiler_lines_2 () readToLinesNonEmpty "helloworld.pro"
    = x 0
    loop x < compiler_lines_2.size {
        var string s () getVecString &compiler_lines_2 x
        = s () stringTrim s
        () println %s s
        ++ x
    }

    ret 0
}
