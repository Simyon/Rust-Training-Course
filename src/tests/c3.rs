use crate::tasks::c3_ownership_and_memory::{
    append_and_return_length, hard_borrowing, last_word, longest_owned, longest_word, print_length,
    simple_borrowing, string_ownership,
};

#[test]
fn test_longest_owned() {
    assert_eq!(
        "🙄🙏🏼😶‍🌫️".to_string(),
        longest_owned("🙄🙏🏼😶‍🌫️".to_string(), "😳🫣🌫🫥".to_string())
    );
    assert_eq!(
        "Arbeit macht frei!".to_string(),
        longest_owned("Arbeit macht frei!".to_string(), "Arbeit macht frei".to_string())
    );
    assert_eq!("ab0ba".to_string(), longest_owned("aboba".to_string(), "ab0ba".to_string()));
}

#[test]
fn test_string_ownership() {
    string_ownership();
}

#[test]
fn test_print_length() {
    print_length(&"1027384956".to_string());
}

#[test]
fn test_simple_borrowing() {
    simple_borrowing();
}

#[test]
fn test_append_and_return_length() {
    assert_eq!(16, append_and_return_length(&mut "I_Hate_Rust_".to_string(), "Joke"));
}

#[test]
fn test_hard_borrowing() {
    hard_borrowing();
}

#[test]
fn test_last_word() {
    assert_eq!("kids", last_word("How do you do fellow kids"));
    assert_eq!("end", last_word(" tricky string with a space at the beginning and end "));
    assert_eq!("spaces", last_word("string     with     excess        spaces"));
    assert_eq!("", last_word(""));
}

#[test]
fn test_longest_word() {
    assert_eq!("there", longest_word("I can't feel my Rust! Bubba its aint there"));
    assert_eq!("Somebody", longest_word("Somebody once told me the world is gonna roll me"));
    assert_eq!("gonna", longest_word("Never gonna give you up Never gonna let you down"));
}
