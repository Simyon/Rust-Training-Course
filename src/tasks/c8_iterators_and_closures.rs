// This chapter is dedicated to the iterators and closures.

use std::collections::HashMap;

// ITERATORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `word_frequencies(text: &str) -> Vec<(String, usize)>` that:
// - Splits the input text into words.
// - Normalizes them to lowercase.
// - Counts how many times each word appears.
// - Returns the result as a vector of `(word, count)` tuples, sorted by descending frequency.
// If some words have the same frequency, return them in alphabetical order.

pub fn word_frequencies(text: &str) -> Vec<(String, usize)> {
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut frequency_map = HashMap::new();
    for word in words {
        let normalized_word = word.to_lowercase();
        *frequency_map.entry(normalized_word).or_insert(0) += 1;
    }

    let mut frequencies: Vec<(String, usize)> = frequency_map.into_iter().collect();

    frequencies.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    frequencies
}

// ----- 2 --------------------------------------
// Write a function `top_k_most_common_letters(text: &str, k: usize) -> Vec<(char, usize)>` that:
// - Counts the frequency of letters only (ignore spaces/punctuation).
// - Case-insensitive.
// - Sorts by descending frequency.
// - Returns the vector with top `k` letters.
// If some letters have the same frequency, return them in alphabetical order.

pub fn top_k_most_common_letters(text: &str, k: usize) -> Vec<(char, usize)> {
    let mut frequency_map = HashMap::new();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let lower_c = c.to_ascii_lowercase();
            *frequency_map.entry(lower_c).or_insert(0) += 1;
        }
    }

    let mut frequency_vec: Vec<(char, usize)> = frequency_map.into_iter().collect();

    frequency_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    frequency_vec.into_iter().take(k).collect()
}

// CLOSURES
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function
// `filter_and_sort_names(names: Vec<String>, minimum_length: usize) -> Vec<String>` that:
// - Filters out names shorter than minimum_length.
// - Sorts the remaining names alphabetically (case-insensitive).
// - Returns the result.
// You must use closures in filtering and sorting.

pub fn filter_and_sort_names(names: Vec<String>, minimum_length: usize) -> Vec<String> {
    let filtered_names: Vec<String> =
        names.into_iter().filter(|name| name.len() >= minimum_length).collect();

    let mut sorted_names = filtered_names;
    sorted_names.sort_by_key(|a| a.to_lowercase());

    sorted_names
}

// ----- 4 --------------------------------------
// Create a function `group_students_by_grade` that:
// - Accepts a `Vec<(String, u32)>` where each tuple is `(student_name, grade)`.
// - Groups students into some map where the key is the grade and the value is a vector of names.
// - Uses closures for grouping logic.
// - Returns the grouped map, sorted internally by student names.

pub fn group_students_by_grade(students: Vec<(String, u32)>) -> HashMap<u32, Vec<String>> {
    let mut groups: HashMap<u32, Vec<String>> = HashMap::new();

    for (name, grade) in students {
        groups.entry(grade).or_default().push(name);
    }

    for (_, names) in groups.iter_mut() {
        names.sort_by_key(|a| a.to_lowercase())
    }

    groups
}
