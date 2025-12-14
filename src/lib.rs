#[path = "tasks/c1_common_concepts.rs"]
pub mod c1_common_concepts;

#[path = "tasks/c3_ownership_and_memory.rs"]
pub mod c3_ownership_and_memory;

#[path = "tasks/c4_structs_methods_enums_pattern_matching.rs"]
pub mod c4_structs_methods_enums_pattern_matching;

#[cfg(test)]
mod tests {
    use crate::c1_common_concepts::*;
    use crate::c3_ownership_and_memory::*;
    use crate::c4_structs_methods_enums_pattern_matching::*;

    #[test]
    fn test_c1_square() {
        assert_eq!(4, square(2));
        assert_eq!(25, square(5));
    }

    #[test]
    fn test_c3_last_word() {
        assert_eq!("world", last_word("hello world"));
        assert_eq!("", last_word(""));
    }

    #[test]
    fn test_c4_fizzbuzz() {
        let result = fizzbuzz(6);
        assert_eq!(vec!["1", "Fizz", "Buzz", "Fizz", "5", "FizzBuzz"], result);
    }
}