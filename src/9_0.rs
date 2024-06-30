fn main() {
    fizzbuzz_to(100);
}

fn is_devisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) {
    for i in 1..=n {
        match (is_devisible_by(i, 3), is_devisible_by(i, 5)) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("{}", i),
        }
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}