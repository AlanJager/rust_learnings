fn main() {
    // variable binding
    let x = 5;

    // expression
    x;
    x + 1;
    15;

    let a = 5u32;

    let y = {
        let x_squared = a * a;
        let x_cube = x_squared * a;

        x_cube + x_squared + a
    };
    
    let z = {
        a * 2;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}