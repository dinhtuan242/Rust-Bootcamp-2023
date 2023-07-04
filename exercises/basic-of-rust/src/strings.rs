// Exercise 1
#[allow(dead_code)]
fn exercise1(color: &str) -> &str {
    color
}

// Exercise 2
// Fix all errors without adding newline
fn exercise2() -> String {
    let mut s = String::from("hello");
    s.push(',');
    s += " world";
    s += "!";
    s
}

// Exercise 3
// Fix errors without removing any line
fn exercise3() -> String {
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &*s2;
    s3
}

// Exercise 4
// Reverse a string

fn reverse_string(input: &str) -> String {
    let mut reversed = String::new();
    let input_bytes = input.as_bytes();

    for i in (0..input_bytes.len()).rev() {
        reversed.push(input_bytes[i] as char);
    }

    reversed
}


// Exercise 5
// Check if a string is a palindrome
fn is_palindrome(word: &str) -> bool {
    let str_bytes = word.as_bytes();
    let str_len = str_bytes.len();
    let mut result = true;

    for i in 0..str_len / 2 {
        if str_bytes[i].to_ascii_lowercase() != str_bytes[str_len - 1 - i].to_ascii_lowercase() {
            result = false;
            break;
        }
    }

    result
}

// Exercise 6
// Count the occurrences of a character in a string
fn count_char_occurrences(string: &str, ch: char) -> usize {
    let mut count_ch = 0;
    let ch_byte = ch as u8;

    for &byte in string.as_bytes() {
        if byte == ch_byte {
            count_ch += 1;
        }
    }
    count_ch
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 1
    #[test]
    fn exercise1_work() {
        assert_eq!("white".to_string(), exercise1("white"));
    }

    // Test for exercise 2
    #[test]
    fn exercise2_work() {
        assert_eq!("hello, world!".to_string(), exercise2());
    }

    // Test for exercise 3
    #[test]
    fn exercise3_work() {
        assert_eq!("hello, world!".to_string(), exercise3());
    }

    // Test for exercise 4
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
        assert_eq!(reverse_string("world"), "dlrow");
        assert_eq!(reverse_string(""), "");
    }

    // Test for exercise 5
    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome("level"), true);
        assert_eq!(is_palindrome("deed"), true);
        assert_eq!(is_palindrome("Rotor"), true);
    }

    // Test for exercise 5
    #[test]
    fn test_non_palindrome() {
        assert_eq!(is_palindrome("hello"), false);
        assert_eq!(is_palindrome("world"), false);
    }

    // Test for exercise 6

    #[test]
    fn test_count_char_occurrences() {
        assert_eq!(count_char_occurrences("Hello", 'l'), 2);
        assert_eq!(count_char_occurrences("Rust is fun", 'u'), 2);
        assert_eq!(count_char_occurrences("Mississippi", 's'), 4);
    }
}