fn main() {
    use std::mem;

    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and stores the borrow and closure in the `print` variable. It will remain borrowed until `print` goes out of scope. `println!` only requires `by reference` so it doesn't impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count` but `&mut count` is less restrictive so it takes that. Immediately borrows `count`.
    let mut  inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    
    inc();

    // The closure still mutably borrows `count` because it is called later. An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    inc();

    let _count_reborrowed = &mut count;

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type would copy into the closure leaving the original untouched. A non-copy must move and so `movable` immediately moves into the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    // `consume` consumes the variable so this would lead to an error. Uncommenting it will cause an error.
    // consume();

    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    
    println!("{}", contains(&1));
    println!("{}", contains(&4));
}