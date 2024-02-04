use std::{cmp::Ordering, io};

use rand::Rng;

// this is a const declaration, const cannot be changed, ever (otherwise they wouldn't be constant)
const _THIS_IS_A_CONST: u32 = 60 * 60 * 60;

fn main() {
    //_guess_game();
    _variable_shadowing();
    _if_expression();
    _awesome_nested_loop();
    _arrays();
    _borrowing();
    _string_slice();
}

fn _guess_game() {
    println!("Guess the number");

    // generate random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        //declare a mutable string to read from the stdin
        // once declared if not mutable, cannot be changed
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //convert the string just read as an integer
        // and continue in case of error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        //compare the typep and the random number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn _variable_shadowing() {
    let _x = 5;
    let _x = _x + 1;

    {
        let _x = _x * 2;
        println!("The value of x in the inner scope is: {_x}");
    }

    println!("The value of x is: {_x}");
}

fn _numeric_types() {
    // f64, that is the default type in case of a  floating point variable
    let _x = 2.0;
    // f32
    let _y: f32 = 3.0;
    //operations
    // addition
    let _sum = 5 + 10;
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;
}

fn _bool() {
    let _t = true; //inferred
    let _f: bool = false; // with explicit type annotation
}

fn _char_type() {
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
}

fn _compound_types() {
    //tuple declaration
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    // or can be declared like this
    let tup = (400, 5, 2);
    //destructuring
    let (_x, _y, _z) = tup;
    let (_x, _y, _) = tup;
    //or just single element
    let _x = tup.0;
    let _y = tup.1;
}

fn _arrays() {
    //all elements must be of the same type and the size is fixed, the data is allocated in the stack
    let _a = [1, 2, 3, 4, 5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    //same as declaring like this:
    // let a = [3, 3, 3, 3, 3];
    let _a = [3; 5];

    //access elements at index using []
    let _first = _a[0];
    let _second = _a[1];

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    //iterate through each value of the array with the for statement
    for month in _months {
        println!("the value is: {month}");
    }

    for number in (1..4).rev() {
        println!("the value is: {number}");
    }
}

fn _function_with_parameters(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn _statements_and_expressions() {
    //Statements are instructions that perform some action and do not return a value.
    //Expressions evaluate to a resultant value
    //let is a statement and thus we can't do this
    //let x = (let y = 6);
    // scope block with curly braces are expression, so we can write this:
    let _y = {
        let x = 3;
        x + 1
    };
}

fn _function_with_return() -> i32 {
    // this if fine
    5
    //this is not
    //5;
}

fn _function_with_return2() -> i32 {
    let x = 2;
    let y = x + 6;
    y //this is to return the value
}

fn _if_expression() {
    let number = 3;
    // if
    // else if
    // else
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    //note that since if it's an expression it can be used like this :
    let _a = if number > 5 { 6 } else { 7 };
    //but not like this:
    // let number = if condition { 5 } else { "six" };
}

fn _loop_example() {
    let mut counter = 0;

    //loop is an expression then we can do this
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn _awesome_nested_loop() {
    let mut count = 0;
    //outer loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        //inner loop
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                //break the inner loop
                break;
            }
            if count == 2 {
                //break to outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn _conditional_loops_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
}

//Stack & Heap (extract from THE book)
// The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out.
// Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size.
// Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
// The heap is less organized: when you put data on the heap, you request a certain amount of space.
// The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
// This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating).
// Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
//Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data;
// that location is always at the top of the stack.

//When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap)
// and the function’s local variables get pushed onto the stack.
// When the function is over, those values get popped off the stack.

//Rust ownership in short :
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn _basic_ownership() {
    // _s is not valid here, it’s not yet declared
    let _s = "hello"; // s is valid from this point forward

    // do stuff with _s
}

fn _borrowing() {
    let s1 = String::from("hello");

    // s1 ownership is not handed to the fn calculate_length
    // so when the fn returns s1 is still valid
    // & is used to get a refererence to the variable
    let len = _calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn _calculate_length(s: &String) -> usize {
    s.len()
}

fn _mutable_reference() {
    let mut s = String::from("hello");
    // passing the reference to a mutable, in the fn we can modify the variable but keeping ownership
    _change(&mut s);
}

fn _change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn _string_slice() {
    let s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];
    println!("{_hello}");
    println!("{_world}");
    println!("{s}");
}
