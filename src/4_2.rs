fn main() {
    let long_lived_binding = 1;

    // This is a block and has a smaller scope than the main function
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` is not in scope in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // variable shadowing

    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding);

        let shadowed_binding = "abc";
        println!("shadowed in inner block: {}", shadowed_binding);
    }

    println!("outside inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}