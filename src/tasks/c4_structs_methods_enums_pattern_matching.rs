// This chapter is dedicated to the structs, methods, enums, and pattern matching

// STRUCTS
// ================================================================================================

// ----- 1 --------------------------------------
// Define a struct `Point` with fields `x` and `y` (both `u32`). Create a function `new_point(x, y)`
// that returns a `Point` instance.

// IMPLEMENT HERE:
struct Point {
    x: u32,
    y: u32,
}

fn new_point(x: u32, y: u32) -> Point {
    Point { x, y }
}

// uncomment once implemented
pub fn point_checker() {
    let point = new_point(3, 4);
    assert_eq!((3, 4), (point.x, point.y));
}

// ----- 2 --------------------------------------
// Define a struct `Rectangle` with width and height. Implement a function
// `can_hold(r1: &Rectangle, r2: &Rectangle) -> bool` that returns true if `r1` can completely
// contain `r2`.

// IMPLEMENT HERE:
#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn can_hold(r1: &Rectangle, r2: &Rectangle) -> bool {
    r1.width >= r2.width && r1.height >= r2.height
}

// uncomment once implemented
pub fn rectangle_checker() {
    let big = Rectangle { width: 10, height: 8 };
    let small = Rectangle { width: 5, height: 4 };

    assert!(can_hold(&big, &small));
    assert!(!can_hold(&small, &big));
}

// METHODS
// ================================================================================================

// ----- 3 --------------------------------------
// Create a `Company` struct with `name: String`, `date_of_origin: u32` and `annual_income: u64`
// fields. Implement `new(...) -> Self` constructor for it and a `total_income(...) -> u64`
// method that would calculate how much money this company earned since it was established
// (excluding taxes).

// IMPLEMENT HERE:
#[allow(dead_code)]
pub struct Company {
    name: String,
    date_of_origin: u32,
    annual_income: u64,
}

impl Company {
    pub fn new(name: String, date_of_origin: u32, annual_income: u64) -> Self {
        Company { name, date_of_origin, annual_income }
    }

    pub fn total_income(&self, current_year: u32) -> u64 {
        let years_in_business = current_year - self.date_of_origin;
        self.annual_income * years_in_business as u64
    }
}

// ----- 4 --------------------------------------
// Define a struct BankAccount with `owner: String` and `balance: u64` fields.
// Implement basic `new(...) -> Self` constructor.
// Implement methods:
// - `deposit(&mut self, amount: u64)` which adds the specified amount to the balance.
// - `withdraw(&mut self, amount: u64) -> bool` which removes the specified amount from the balance
//   and returns `true`, or just returns `false` if there are insufficient funds.
// - `balance(&self) -> u64` which returns the current balance.

#[allow(dead_code)]
pub struct BankAccount {
    owner: String,
    balance: u64,
}

impl BankAccount {
    pub fn new(owner: String, balance: u64) -> Self {
        BankAccount { owner, balance }
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: u64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }
}

// ENUMS
// ================================================================================================

// ----- 5 --------------------------------------
// Define an enum `TrafficLight` with variants `Red`, `Yellow`, and `Green`. Implement a
// `next(light: &TrafficLight) -> TrafficLight` method for it that returns the next light in
// sequence.

#[derive(Debug, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn next(light: &TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Red => TrafficLight::Green,
        TrafficLight::Yellow => TrafficLight::Red,
        TrafficLight::Green => TrafficLight::Yellow,
    }
}

// ----- 6 --------------------------------------
// Define an enum `Operation` with variants `Add(i32, i32)`, `Subtract(i32, i32)`,
// `Multiply(i32, i32)` and `Divide(i32, i32)`. Implement a method
// `apply(self) -> Option<i32>` for it that computes the result and returns `None` if
// dividing by zero (you can use `match` for convenience)

#[allow(dead_code)]
pub enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

impl Operation {
    pub fn apply(self) -> Option<i32> {
        match self {
            Operation::Add(a, b) => Some(a + b),
            Operation::Subtract(a, b) => Some(a - b),
            Operation::Multiply(a, b) => Some(a * b),
            Operation::Divide(a, b) => {
                if b == 0 {
                    None
                } else {
                    Some(a / b)
                }
            },
        }
    }
}

// PATTERN MATCHING
// ================================================================================================

// ----- 7 --------------------------------------
// Write a enum `WeirdLengthMeasures`, containing `Inch`, `Foot`, `Yard` and `Mile` variants.
// Implement a `convert_to_human_format(&self) -> f64` method, which returns the length of of the
// provided weirdo lengths in meters using pattern matching (with `match`).
// Use provided values:
// - Inch -> 0.0254 m
// - Foot -> 0.3048 m
// - Yard -> 0.9144 m
// - Mile -> 1609.344 m

#[allow(dead_code)]
pub enum WeirdLengthMeasures {
    Inch,
    Foot,
    Yard,
    Mile,
}

impl WeirdLengthMeasures {
    pub fn convert_to_human_format(&self) -> f64 {
        match self {
            WeirdLengthMeasures::Inch => 0.0254,
            WeirdLengthMeasures::Foot => 0.3048,
            WeirdLengthMeasures::Yard => 0.9144,
            WeirdLengthMeasures::Mile => 1609.344,
        }
    }
}

// ----- 8 --------------------------------------
// Write a function `fizzbuzz(n: u32) -> Vec<String>` that returns a vector of strings from 1 to n
// where:
// - Multiples of 2 are "Fizz",
// - Multiples of 3 are "Buzz",
// - Multiples of both 2 and 3 are "FizzBuzz",
// - Otherwise the number itself.

pub fn fizzbuzz(n: u32) -> Vec<String> {
    (1..=n)
        .map(|i| match (i % 2, i % 3) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => i.to_string(),
        })
        .collect()
}
