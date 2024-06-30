fn main() {
    // let triple = (0, -2, 3);
    // let triple = (1, -2, 3);
    // let triple = (3, 0, 2);
    // let triple = (3, 0, 4);
    let triple = (5, 0, 4);

    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("The last is `2`, and the rest doesn't matter"),
        (3, .., 4) => println!("The first is `3`, and the last is `4`, and the middle doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
}