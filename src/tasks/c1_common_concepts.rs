// This chapter is dedicated to the common programming concepts, like variables and their
// mutability, data types, functions and control flow stuff

// MUTABILITY
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function that declares a mutable integer variable, assigns it the value 5, then changes
// it to 10, and prints both values.
#[allow(dead_code)]
pub fn simple_mutability() {
    let mut number = 5;
    println!("The number is: {}", number);
    number = 10;
    println!("The number is: {}", number);
}

// DATA TYPES
// ================================================================================================

// ----- 2 --------------------------------------
// Create variables of types `i32``, `f64``, `bool``, and `char``, assign them values, and print
// them.
#[allow(dead_code)]
pub fn simple_data_types() {
    //Does the Rust really don't have onliner declaration?
    let i32_int: i32 = 5;
    let f64_float: f64 = 3.14159265357;
    let bool_bool: bool = false;
    let char_char: char = '@';

    println!(
        "The i32 number is: {},\t f64 number is: {},\t bool is: {},\t char is: {}",
        i32_int, f64_float, bool_bool, char_char
    );
}

// FUNCTIONS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `square` that takes a `u32` integer and returns its square as `u32`.

// IMPLEMENT HERE:
#[allow(dead_code)]
pub fn square(u32_int: u32) -> u32 {
    u32_int * u32_int
}

// ----- 4 --------------------------------------
// Write a recursive function `factorial` that computes the factorial of a number (n!) as `u32`.

// IMPLEMENT HERE:
#[allow(dead_code)]
pub fn factorial(u32_int: u32) -> u32 {
    // No ternary operator in Rust :(
    if u32_int == 0 {
        1
    } else {
        u32_int * factorial(u32_int - 1)
    }
}

// CONTROL FLOW
// ================================================================================================

// ----- 5 --------------------------------------
// Write a program that prints whether a provided signed integer number is positive, negative, or
// zero using `if` statement.
#[allow(dead_code)]
#[allow(clippy::comparison_chain)] // According to task I have to using 'if' statement
pub fn sign_checker(number: i32) -> &'static str {
    if number > 0 {
        "positive"
    } else if number < 0 {
        "negative"
    } else {
        "zero"
    }
}

// ----- 6 --------------------------------------
// Write a program that finds the largest number in an array of 5 integers using a for or while
// loop.
#[allow(clippy::needless_range_loop)] // I suppose that I don't know iterators at first task
pub fn find_biggest_number(some_array: [u32; 5]) -> u32 {
    let mut max_number = some_array[0];
    for i in 1..5 {
        if some_array[i] > max_number {
            max_number = some_array[i];
        }
    }
    max_number
}
