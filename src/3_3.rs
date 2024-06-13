fn main() {
    println!("Hello, world!");

    another_function();
    another_function1(5);
    print_label_measurement(5, 'm');

//     let y = 6;
//     let x = (let y = 6);
//     println!("The value of x is: {}", x);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(x);
    println!("The value of x is: {}", x);
}

// fn plus_one(x: i32) -> i32 {
//     x + 1;
// }

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_function() {
    println!("Another function.");
}

fn another_function1(x: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The value of x is: {}{}", value, unit_label);
}