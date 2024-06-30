fn main() {
    let array = [1, -2, 6];

    match array {
        [0, seconds, third] =>
            println!("array[0]: 0, array[1]: {}, array[2]: {}", seconds, third),
        [1, _, third] => println!(
            "array[0]: 1, array[2]: {}",
            third
        ),
        [-1, second, ..] => println!(
            "array[0]: -1, array[1]: {}",
            second
        ),
        [3, second, tail @ ..] => println!(
            "array[0]: 3, array[1]: {}, array[2..]: {:?}",
            second, tail
        ),
        [first, middle @ .., last] => println!(
            "array[0]: {}, array[1..]: {:?}, array[2]: {}",
            first, middle, last
        ),
    }
}