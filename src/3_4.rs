fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    let number = 3;

    // if number {
    //     println!("Number was three");
    // }
    if number != 0 {
        println!("Number was something other than zero");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // let condition = true;
    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {}", number);

    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    let mut counter = 0;
    'couting_up: loop {
        println!("counter is: {}", counter);
        
        let mut remaining = 10;
        loop {
            println!("remaining is: {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'couting_up;
            }
            remaining -= 1;
        }

        counter += 1;
    }
    println!("End counter is: {}", counter);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for number in (1..=4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}