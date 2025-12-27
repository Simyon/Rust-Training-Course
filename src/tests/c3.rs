use crate::tasks::c3_ownership_and_memory::{longest_owned, string_ownership};

#[test]
fn test_longest_owned() {
    assert_eq!("🙄🙏🏼😶‍🌫️".to_string(), longest_owned("🙄🙏🏼😶‍🌫️".to_string(), "😳🫣🌫🫥".to_string()));
    assert_eq!("Arbeit macht frei!".to_string(), longest_owned("Arbeit macht frei!".to_string(), "Arbeit macht frei".to_string()));
    assert_eq!("ab0ba".to_string(), longest_owned("aboba".to_string(), "ab0ba".to_string()));
}

#[test]
fn test_string_ownership() {
    string_ownership();
}

/*
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
*/
