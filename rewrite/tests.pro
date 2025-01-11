import utils.pro

fn t_testme {
    var int val () testme
    () assertInt val 0
}

fn main {
    () test t_testme "testme"

    () print "\nPassed all ["
    () print %i passed_tests
    () println "] tests."
}
