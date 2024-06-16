fn main() {
    let elem = 5u8;

    // create an empty vector (a growable array)    
    let mut vec = Vec::new();

    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`)
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)

    println!("{:?}", vec);
}