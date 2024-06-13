use std::any::type_name;
use std::io;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    let guess: u16 = "42".parse().expect("Not a number!");
    let guess: u8 = "42".parse().expect("Not a number!");
    let guess: u64 = "42".parse().expect("Not a number!");
    let guess: u128 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = 5 + 10;
    // print value and its type
    println!("The value of sum is: {} type is: {}", sum, type_of(sum));

    let difference = 95.5 - 4.3;
    println!("The value of difference is: {} type is: {}", difference, type_of(difference));

    let product = 4 * 30;
    println!("The value of product is: {} type is: {}", product, type_of(product));

    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {} type is: {}", quotient, type_of(quotient));

    let truncated = -5 / 3;
    println!("The value of truncated is: {} type is: {}", truncated, type_of(truncated));

    let remainder = 43 % 5;
    println!("The value of remainder is: {} type is: {}", remainder, type_of(remainder));

    let t = true;
    let f: bool = false;

    println!("The value of t is: {} type is: {}", t, type_of(t));
    println!("The value of f is: {} type is: {}", f, type_of(f));

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("The value of c is: {} type is: {}", c, type_of(c));
    println!("The value of z is: {} type is: {}", z, type_of(z));
    println!("The value of heart_eyed_cat is: {} type is: {}", heart_eyed_cat, type_of(heart_eyed_cat));

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?} type is: {}", tup, type_of(tup));

    let (x, y, z) = tup;

    println!("The value of x is: {} type is: {}", x, type_of(x));
    println!("The value of y is: {} type is: {}", y, type_of(y));
    println!("The value of z is: {} type is: {}", z, type_of(z));

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of five_hundred is: {} type is: {}", five_hundred, type_of(five_hundred));
    println!("The value of six_point_four is: {} type is: {}", six_point_four, type_of(six_point_four));
    println!("The value of one is: {} type is: {}", one, type_of(one));

    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?} type is: {}", a, type_of(a));

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];
    println!("The value of a is: {:?} type is: {}", a, type_of(a));

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {} type is: {}", first, type_of(first));
    println!("The value of second is: {} type is: {}", second, type_of(second));

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    let guess = io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Please type a number!");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}