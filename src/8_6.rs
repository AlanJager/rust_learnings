enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

enum Foo2 {Bar}

fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        },
        _ => {},
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable `b` matches `Foo::Baz`
    // So this will print "b is foobar"
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Variable `c` matches `Foo::Qux` which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }

    let d = Foo2::Bar;
    if let Foo2::Bar = d {
        println!("d is foobar");
    }
}