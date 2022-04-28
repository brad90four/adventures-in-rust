// time to grind


// debug printing is love, debug printing is life
#[derive(Debug)] // this automatically creates the fmt::Debug implementation for the below type
struct MyStruct { // define a struct
    a: i32,
    description: str,
}


fn main() {
    println!("Hello, world!");

    // create a struct from MyStruct
    let my_struct = MyStruct {
        a: 1,
        description: "This is a struct",
    };

    println!("{:#?}", my_struct); // {:#?} is pretty printing, {:?} is debug printing

    // basic types
    let a_norm_int: i32 = 1234;
    let a_norm_float: f32 = 1234.5;
    let mut change_int: i32 = 100;
    change_int = 200;

    // if statements
    if a_norm_int > change_int: {
        println!("{} is greater than {}", a_norm_int, change_int);
    } else {
        println!("{} is less than {}", a_norm_int, change_int);
    }
    // logical operators
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // loop de loop
    let result = loop {
        let mut count: u32 = 0;

        println!("count is {}", count);

        if count == 3 {
            println!("stopping the loop");
            break count; // break the loop and retun the value
        }

        count += 1;
    }; // need to include the semicolon with assignment loop

    // loops, but with a label
    'outer: loop {
        println!("entering the outer loop");

        'inner: loop {
            println!("entering the inner loop");

            break 'outer;
        }

        println!("this point will never be reached");
    }

    // for loops
    for iterator in 0..=3 { // ..= is inclusive
        if iterator % 2 == 0 {
            println!("{} is even", iterator);
        } else {
            println!("{} is odd", iterator);
        }
    }

    // next up - https://doc.rust-lang.org/rust-by-example/flow_control/match.html

}


