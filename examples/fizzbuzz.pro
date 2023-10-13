// nostd

// Fizzbuzz implementation
fn fizzbuzz int max > void {
    var int i 1
    loop i <= max {
        if i % 15 == 0 {
            () println "FizzBuzz"
        }
        elif i % 3 == 0 {
            () println "Fizz"
        }
        elif i % 5 == 0 {
            () println "Buzz"
        }
        else {
            () println %d i
        }
        ++ i
    }
}

fn main {
    () fizzbuzz 20
}
