use core::panic;
use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Invalid input: {}", s);
    };

    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Invalid count: {}", count_str);
    };
    (count, item)
}

fn main() {
    assert_eq!(get_count_item("4 apples"), (4, "apples"));
}