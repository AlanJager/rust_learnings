use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl Into<i32> for Number {
    fn into(self) -> i32 {
        self.value
    }
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    println!("my_str: {}", my_str);
    println!("my_string: {}", my_string);

    let num = Number::from(30);
    println!("My nymber is {:?}", num);

    let int = 5;

    let num: Number = int.into();
    println!("My number is {:?}", num);
}