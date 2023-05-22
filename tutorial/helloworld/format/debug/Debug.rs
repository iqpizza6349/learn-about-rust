// #[derive(Debug)] 가 있을 경우, 자동으로 `fmt:Debug` 를 구현해준다고 한다.
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

// Quick Structure to print example!
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // `{:?}` is similar to with `{}`
    println!("{:?} month in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);
}
