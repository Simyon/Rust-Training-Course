// This chapter is dedicated to the error handling, tests and documentation.

// RESULT
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `first_char(text: &str) -> Result<char, String>` that returns the first
// character of a string or an error message "Empty string" if the string is empty.

pub fn first_char(text: &str) -> Result<char, String> {
    match text.chars().next() {
        Some(c) => Ok(c),
        None => Err("Empty string".to_string()),
    }
}

// ----- 2 --------------------------------------
// Write a function `read_numbers_from_str(line: &str) -> Result<Vec<i32>, String>` that reads a
// line of integers separated by whitespace and parses each integer as i32. In case the value cannot
// be parsed (if it is not an integer) return the `Err("Invalid number")` result.

pub fn read_numbers_from_str(line: &str) -> Result<Vec<i32>, String> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().map_err(|_| "Invalid number".to_string()))
        .collect()
}

// OPTION
// ================================================================================================

// ----- 3 --------------------------------------
// You have a struct `UserProfile` with fields `username: String` and `email: Option<String>`.
//
// Implement a method `get_email_domain(&self) -> Option<String>` that:
// - If the email exists, extracts the domain (the part after @).
// - If the email is missing, returns `None`.

// IMPLEMENT HERE:
pub struct UserProfile {
    #[allow(dead_code)]
    username: String,
    email: Option<String>,
}

impl UserProfile {
    pub fn new(username: String, email: Option<String>) -> Self {
        UserProfile { username, email }
    }

    pub fn get_email_domain(&self) -> Option<String> {
        self.email
            .as_ref()
            .and_then(|email| email.rsplit('@').next().map(|domain| domain.to_string()))
    }
}

// WRITING TESTS
// ================================================================================================

// ----- 4 --------------------------------------
// Write unit tests for the `factorial(n: u32) -> u64` function.

fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

#[cfg(test)]
mod factorial_tests {
    use super::*;

    #[test]
    fn test_factorial_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_1() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn test_factorial_5() {
        assert_eq!(factorial(5), 120);
    }
}

// ----- 5 --------------------------------------
// Write unit tests for the `is_prime(n: u64) -> bool` function checking both prime and non-prime
// numbers.

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    for divisor in 2..=((number as f64).sqrt() as u64) {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod prime_tests {
    use super::*;

    #[test]
    fn test_is_prime_2() {
        assert!(is_prime(2));
    }

    #[test]
    fn test_is_prime_57() {
        assert!(!is_prime(57));
    }

    #[test]
    fn test_is_prime_129() {
        assert!(!is_prime(129));
    }

    #[test]
    fn test_is_prime_1000000007() {
        assert!(is_prime(1000000007));
    }

    #[test]
    fn test_is_prime_17() {
        assert!(is_prime(17));
    }
}

// WRITING DOCS
// ================================================================================================

// ----- 6 --------------------------------------
// You have an implemented `TemperatureLog` struct below, which stores a city name and a list of
// daily temperature readings. This struct have a constructor, an `add_reading` method which just
// ads a new value to the `readings` vector and an `average` method which returns an average value
// of the readings of there are some.
//
// Your task is to add doc comments:
// - High-level purpose of the struct.
// - Inline docs for each field and method.
//
// In case you want something more than хор(5):
// - Additionally white the usage example for the `TemperatureLog` in the high-level docs.
// - For the `average` method additionally write an example of its usage.

/// Лог температур по дням для одного города.
///
/// # Пример
/// ```
/// use crate::TemperatureLog;
///
/// let mut log = TemperatureLog::new("Moscow");
/// log.add_reading(-10.5);
/// log.add_reading(-11.5);
/// assert_eq!(log.average(), Some(-11.0));
/// ```
#[allow(dead_code)]
pub struct TemperatureLog {
    /// Название города, для которого регистрируются температуры.
    pub city: String,
    /// Список температуры по цельсию.
    pub readings: Vec<f64>,
}

#[allow(dead_code)]
impl TemperatureLog {
    /// Создает новый `TemperatureLog` для указанного города.
    pub fn new(city: &str) -> Self {
        Self {
            city: city.to_string(),
            readings: Vec::new(),
        }
    }

    /// Добавляет запись температуры в лог.
    pub fn add_reading(&mut self, value: f64) {
        self.readings.push(value);
    }

    /// Вычисление средней температуры по записям.
    ///
    /// # Пример
    ///
    /// ```
    /// let mut log = TemperatureLog::new("Moscow");
    /// log.add_reading(20.5);
    /// log.add_reading(22.0);
    /// assert_eq!(log.average(), Some(21.25));
    /// ```
    pub fn average(&self) -> Option<f64> {
        if self.readings.is_empty() {
            return None;
        }
        let sum_of_readings: f64 = self.readings.iter().sum();
        Some(sum_of_readings / self.readings.len() as f64)
    }
}
