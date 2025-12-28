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
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

#[allow(dead_code)]
pub fn string_ownership() {
    let a = String::from("a");
    let b = String::from("b");
    let c = longest_owned(a, b);

    //println!("The previous strings: {}, {} are moved into the function, so they are no longer valid here. Uncomment'll invoke compilation error", a, b);
    println!("The longest string is {}", c);
}

// BORROWING
// ================================================================================================

// ----- 2 --------------------------------------
// Write a function `print_length(s: ???)` that takes some string and prints its length without
// taking ownership. First use it with some random (censored) string, and then print this string to
// show that it was not moved and still available.
//
// You can implement the function and use it right inside the `simple_borrowing` function.
pub fn print_length(s: &String){
    println!("The length of the string is {}", s.len());
}

#[allow(dead_code)]
pub fn simple_borrowing() {
    let s : String = "count me ~nipa".to_string();
    print_length(&s);
    println!("The string is still here: {}", s);
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.
//
// You can implement the function and use it right inside the `hard_borrowing` function.
pub fn append_and_return_length(string: &mut String, suffix: &str) -> usize {
    string.push_str(suffix);
    string.len()
}

#[allow(dead_code)]
pub fn hard_borrowing() {
    let mut base_string = String::from("4el");
    println!("The length of the string is {}", append_and_return_length(&mut base_string, " ty"));
    println!("The length of the string is {}", append_and_return_length(&mut base_string, " w"));
    println!("The length of the string is {}", append_and_return_length(&mut base_string, " mute."));
}

// SLICES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.
pub fn last_word(slice: &str) -> &str {
    //println!("slice: {}", slice);
    if slice.len() == 0 {
        return "";
    }
    let mut index = 0;
    let mut space_index = 0;
    let mut unspace_index : usize = 0;
    'outter_loop: loop {
        //println!("index: {}\tspace_index: {}\tunspace_index: {}", index, space_index, unspace_index);
        if slice.as_bytes()[index] == b' ' {
            space_index = index;
        } else {
            unspace_index = index;
        }
        index += 1; // Why Rust has no postfix increment operator;-(
        if index == slice.len() {
            //println!("Last index: {}\tspace_index: {}\tunspace_index: {}", index, space_index, unspace_index);
            if space_index > unspace_index {
                space_index = unspace_index;
                'inner_loop: loop {
                    if slice.as_bytes()[space_index] != b' ' {
                        space_index -= 1;
                    } else {
                        break 'inner_loop;
                    }
                }
            }
            return &slice[space_index + 1..unspace_index + 1];
        }
    }
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.
pub fn longest_word(sentence: &str) -> &str {
    //println!("sentence: {}", sentence);
    if sentence.len() == 0 {
        return "";
    }
    let mut index = 0;
    let mut space_index = 0;
    let mut unspace_index : usize = 0;
    let mut max_length = 0; 
    let mut start_max_word_index = 0;
    let mut end_max_word_index = 0;
    'outter_loop: loop {
        //println!("index: {}\tspace_index: {}\tunspace_index: {}", index, space_index, unspace_index);
        if sentence.as_bytes()[index] == b' ' {
            space_index = index;
            if space_index > unspace_index && space_index - unspace_index == 1 {
                space_index = unspace_index;
                'inner_loop: loop {
                    //println!("In inner_loop space_index: {}\tunspace_index: {}", space_index, unspace_index);
                    if space_index == 0 {
                        break 'inner_loop;
                    }
                    if sentence.as_bytes()[space_index] != b' ' {
                        space_index -= 1;
                    } else {
                        break 'inner_loop;
                    }                    
                }
                let length = unspace_index - space_index + 1;
                if length >= max_length {
                    max_length = length;
                    if space_index == 0 {
                        start_max_word_index = 0;
                    } else {
                        start_max_word_index = space_index + 1;
                    }
                    end_max_word_index = unspace_index + 1;
                }
            }
            space_index = index;
        } else {
            unspace_index = index;
        }
        index += 1; 
        if index == sentence.len() {
            //println!("Last index: {}\tspace_index: {}\tunspace_index: {}", index, space_index, unspace_index);
            if space_index > unspace_index {
                space_index = unspace_index;
                'inner_loop: loop {
                    if sentence.as_bytes()[space_index] != b' ' {
                        space_index -= 1;
                    } else {
                        break 'inner_loop;
                    }
                }
            }
            let length = unspace_index - space_index + 1;
            //println!("length: {}\tmax_length: {}", length, max_length);
            if length >= max_length {
                max_length = length;
                start_max_word_index = space_index + 1;
                end_max_word_index = unspace_index + 1;
            }
            return &sentence[start_max_word_index..end_max_word_index];
        }
    }
}
