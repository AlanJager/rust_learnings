#[allow(dead_code)]
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

fn main() {
    let temperature = Temperature::Celsius(42.0);

    match temperature {
        Temperature::Celsius(c) => {
            let f = c * 1.8 + 32.0;
            println!("{}°C is {}°F", c, f);
        },
        Temperature::Fahrenheit(f) => {
            let c = (f - 32.0) / 1.8;
            println!("{}°F is {}°C", f, c);
        },
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Less than zero"),
    }
}