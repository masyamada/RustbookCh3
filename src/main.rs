// "let the mutable variable x be equal to 5"
// let mut x = 5; // writable, initially valued at 5
// "the contant variable x is of type u32 and is equal to 17 times 5"
// const x: u32 = 17*5; // parsed by compiler and is immutable
//
use std::io;

fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Now look at arrays (tuples is skipped for now)

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    if index > 4 {
        println!("got passed array index")
    };

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // functions
    another_function(index.try_into().unwrap());

    // looping in initialiation
    let p: i32 = loop {
        let mut _inew = String::new();

        println!("give me an integer, please.");

        io::stdin()
            .read_line(&mut _inew)
            .expect("Failed to read line");

        let _inew: i32 = _inew
            .trim()
            .parse()
            .expect("String entered was not an  integer");

        break _inew
    };

    println!("you entered the number {}", p);

    let ibetter = read_my_int();

    println!("you entered the number {}", ibetter);

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn read_my_int() -> i32 {
    loop {
        let mut _inew = String::new();

        println!("give me an integer, please.");

        io::stdin()
            .read_line(&mut _inew)
            .expect("Failed to read line");

        let _inew: i32 = _inew
            .trim()
            .parse()
            .expect("String entered was not an  integer");

        break _inew
    }
}
