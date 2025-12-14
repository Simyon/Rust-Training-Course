#[path = "../tasks/c3_ownership_and_memory.rs"]
mod c3_ownership_and_memory;

use c3_ownership_and_memory::{
    last_word, longest_word, longest_owned, 
    append_and_return_length, print_length,
    string_ownership, simple_borrowing, hard_borrowing
};

fn test_last_word() {
    println!("Тестируем last_word...");
    
    assert_eq!("kids", last_word("How do you do fellow kids"));    
    assert_eq!("end", last_word(" tricky string with a space at the beginning and end "));    
    assert_eq!("spaces", last_word("string     with     excess        spaces"));    
    assert_eq!("", last_word(""));
    assert_eq!("", last_word("   "));    
    assert_eq!("word", last_word("word"));
    assert_eq!("word", last_word("  word  "));
    
    println!("Все тесты last_word прошли!\n");
}

fn test_longest_word() {
    println!("Тестируем longest_word...");
    
    assert_eq!("there", longest_word("I can't feel my Rust! Bubba its aint there"));    
    assert_eq!("Somebody", longest_word("Somebody once told me the world is gonna roll me"));    
    assert_eq!("gonna", longest_word("Never gonna give you up Never gonna let you down"));    
    assert_eq!("", longest_word(""));
    assert_eq!("word", longest_word("word"));
    
    println!("Все тесты longest_word прошли!\n");
}

fn test_longest_owned() {    
    let s1 = String::from("короткая");
    let s2 = String::from("очень длинная строка");
    let result = longest_owned(s1, s2);
    assert_eq!("очень длинная строка", result);
    
    let s3 = String::from("abcd");
    let s4 = String::from("efgh");
    let result2 = longest_owned(s3, s4);
    assert_eq!("abcd", result2);
    
    println!("Все тесты longest_owned прошли!\n");
}

fn test_append_and_return_length() {
    println!("Тестируем append_and_return_length (изменяемое заимствование)...");
    
    let mut test_string = String::from("Hello");
    
    let len1 = append_and_return_length(&mut test_string, " World");
    assert_eq!(11, len1);
    assert_eq!("Hello World", test_string);
    
    let len2 = append_and_return_length(&mut test_string, "!");
    assert_eq!(12, len2);
    assert_eq!("Hello World!", test_string);
    
    let len3 = append_and_return_length(&mut test_string, "");
    assert_eq!(12, len3);
    assert_eq!("Hello World!", test_string);
    
    println!("Все тесты append_and_return_length прошли!\n");
}

fn test_print_length() {
    println!("Тестируем print_length (простое заимствование)...");
    
    print_length("test string");
    print_length("");
    print_length("очень длинная строка с русскими символами");
    
    println!("Все тесты print_length прошли!\n");
}

fn main() {
    println!("=== 🦀 Тестирование главы 3: Владение и память ===\n");
    
    println!("Демонстрация владение строками:");
    string_ownership();
    println!();
    
    println!("Демонстрация простого заимствования:");
    simple_borrowing();
    println!();
    
    println!("Демонстрация изменяемого заимствования:");
    hard_borrowing();
    println!();    
    
    test_last_word();
    test_longest_word();
    test_longest_owned();
    test_append_and_return_length();
    test_print_length();
    
    println!("Все тесты главы 3 пройдены!");
}