use rand::seq::SliceRandom;

/// Selects a single random word from a slice of words.
///
/// # Arguments
///
/// * `words` - A slice of String objects to choose from. This is a borrow.
///
/// # Returns
///
/// An `Option<&str>` which is:
/// - `Some(&str)` containing a reference to a random word if the slice is not empty.
/// - `None` if the input slice is empty.
pub fn get_random_word(words: &[String]) -> Option<&str> {
    // .choose() is a convenient method from the SliceRandom trait.
    // It picks one random element from a slice and returns it as an Option,
    // which is Some(&item) if the slice is not empty, and None if it is.
    // It returns an Option<&String>, so we use .map() to convert the inner
    // value from &String to &str.
    words.choose(&mut rand::thread_rng()).map(String::as_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_from_list() {
        // Create a vector of owned Strings for our test.
        let words = vec![
            "apple".to_string(),
            "banana".to_string(),
            "mango".to_string(),
        ];

        // Call our function, passing a slice (a borrow) of the vector.
        let random_word_option = get_random_word(&words);

        // We should definitely get a word back.
        assert!(random_word_option.is_some());
        let random_word = random_word_option.unwrap(); // Safe to unwrap here.

        // Check that the word it returned is one of the ones we provided.
        assert!(words.iter().any(|w| w == random_word));
    }

    #[test]
    fn test_get_from_empty_list() {
        let empty_words: Vec<String> = vec![];
        let result = get_random_word(&empty_words);

        // Assert that the function correctly returns None for an empty list.
        assert_eq!(result, None);
    }
}