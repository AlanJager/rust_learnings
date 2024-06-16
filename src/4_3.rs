fn main() {
    let a_abinding;
    {
        let x = 2;
        a_abinding = x * x;
    }

    println!("a binding: {}", a_abinding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}