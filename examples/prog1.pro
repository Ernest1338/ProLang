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

    ret 0
}
