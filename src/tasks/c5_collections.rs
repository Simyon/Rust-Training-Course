// This chapter is dedicated to some collections: vectors, strings and hash maps

use std::collections::{HashMap, HashSet};

// VECTORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `second_largest(vec: &[i32]) -> Option<i32>` that returns the second largest
// element in the array. If the array has fewer than 2 elements, return `None`.

pub fn second_largest(vec: &[i32]) -> Option<i32> {
    if vec.len() < 2 {
        return None;
    }

    let mut sorted_vec = vec.to_vec();
    sorted_vec.sort();
    sorted_vec.dedup();

    if sorted_vec.len() < 2 {
        return None;
    }

    Some(*sorted_vec.get(sorted_vec.len() - 2)?)
}

// ----- 2 --------------------------------------
// Write a function `longest_increasing_subsequence(vec: &[i32]) -> Vec<i32>`` that finds the
// longest strictly increasing subsequence (not necessarily contiguous) in the array.
//
// For the simplicity, assume that there is only one longest increasing subsequence.

pub fn longest_increasing_subsequence(init_sequence: &[i32]) -> Vec<i32> {
    if init_sequence.is_empty() {
        return Vec::new();
    }

    let n = init_sequence.len();
    let mut dp = vec![1; n]; // dp[i] = длина НВП на i
    let mut prev = vec![None; n]; // prev[i] = индекс предыдущего элемента в НВП

    for i in 1..n {
        for j in 0..i {
            if init_sequence[j] < init_sequence[i] && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                prev[i] = Some(j);
            }
        }
    }

    // Находим индекс с максимальной длиной НВП
    let (last_idx, &max_len) = dp.iter().enumerate().max_by_key(|&(_, &len)| len).unwrap();

    // Получаем НВП
    let mut lis = Vec::with_capacity(max_len);
    let mut current = Some(last_idx);
    while let Some(idx) = current {
        lis.push(init_sequence[idx]);
        current = prev[idx];
    }
    lis.reverse();

    lis
}

// STRINGS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `reverse_words(sentence: &str) -> String` that reverses the order of words in a
// sentence but does not reverse the characters inside each word.

pub fn reverse_words(sentence: &str) -> String {
    sentence.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

// ----- 4 --------------------------------------
// Write a function `normalize_and_capitalize(sentence: &str) -> String` that:
// - Trims extra spaces at the beginning and end.
// - Converts multiple spaces between words into a single space.
// - Makes the first letter of every word uppercase, and every other letter lowercase, for example
//   "пРеВеД МеДвЕд -> Превед Медвед"

pub fn normalize_and_capitalize(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>()
                        + &chars.map(|c| c.to_lowercase().to_string()).collect::<String>()
                },
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

// HASH SET
// ================================================================================================

// ----- 5 --------------------------------------
// Write a function `unique_chars(s: &str) -> bool` that returns true if a string has all unique
// characters (ignoring case), and false otherwise.

pub fn unique_chars(s: &str) -> bool {
    let mut seen = HashSet::new();
    for ch in s.chars().map(|c| c.to_lowercase().next().unwrap_or(c)) {
        if !seen.insert(ch) {
            return false;
        }
    }
    true
}

// HASH MAP
// ================================================================================================

// ----- 6 --------------------------------------
// Write a function `top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32>` that returns the `k` most
// frequent numbers in the vector. If `k` is greater than the total number of unique elements in the
// vector, return all of them.

pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut frequency_map = HashMap::new();
    for &num in &nums {
        *frequency_map.entry(num).or_insert(0) += 1; // Считаем частоту каждого числа
    }

    let mut freq_vec: Vec<_> = frequency_map.into_iter().collect();

    freq_vec.sort_by(|a, b| b.1.cmp(&a.1)); // Сортируем по убыванию частоты

    // Извлекаем первые k чисел (или все, если k больше количества уникальных элементов)
    freq_vec.into_iter().take(k).map(|(num, _)| num).collect()
}
