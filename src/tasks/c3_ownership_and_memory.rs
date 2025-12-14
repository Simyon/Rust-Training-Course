// This chapter is dedicated to the ownership, borrowing and slices

// OWNERSHIP
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `longest_owned(s1: String, s2: String) -> String` that returns the longer of
// two strings. Check that both original strings are moved into the function, and only the returned
// can still be used.
//
// You can implement the function and use it right inside the `string_ownership` function.
pub fn longest_owned(s1: String, s2: String) -> String {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

#[allow(dead_code)]
pub fn string_ownership() {
    let s1 = String::from("аб0ба");
    let s2 = String::from("abracadabra");
    let result = longest_owned(s1, s2);
    println!("Самая длинная строка: {}", result);
}

// BORROWING
// ================================================================================================

// ----- 2 --------------------------------------
// Write a function `print_length(s: ???)` that takes some string and prints its length without
// taking ownership. First use it with some random (censored) string, and then print this string to
// show that it was not moved and still available.
//
// You can implement the function and use it right inside the `simple_borrowing` function.
// Функция принимает ссылку на строку (&str), не забирая владение
pub fn print_length(s: &str) {
    println!("Длина строки: {}", s.len());
}

#[allow(dead_code)]
pub fn simple_borrowing() {
    let my_string = String::from("секретная строка");
    print_length(&my_string);
    println!("Строка все еще доступна: {}", my_string);
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.
//
// You can implement the function and use it right inside the `hard_borrowing` function.
// Функция принимает изменяемую ссылку на String и неизменяемую ссылку на str
pub fn append_and_return_length(string: &mut String, suffix: &str) -> usize {
    string.push_str(suffix);
    string.len()
}

#[allow(dead_code)]
pub fn hard_borrowing() {
    let mut my_string = String::from("Привет");
    let len1 = append_and_return_length(&mut my_string, ", мир");
    println!("После первого добавления: '{}', длина: {}", my_string, len1);    
    let len2 = append_and_return_length(&mut my_string, "!");
    println!("После второго добавления: '{}', длина: {}", my_string, len2);
    println!("Финальная строка: {}", my_string);
}

// SLICES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.
pub fn last_word(slice: &str) -> &str {
    // Убираем пробелы с концов и разбиваем по пробелам, потом берём последнее слово и если строка пустая, возвращаем пустую строку
    slice.trim()
        .split_whitespace()
        .last()  
        .unwrap_or("")   
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.
pub fn longest_word(sentence: &str) -> &str {
    sentence
        .split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap_or("")
}