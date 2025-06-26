use rand::seq::SliceRandom;

/// Selects a single random word from a slice of words, using a smaller, more common
/// subset of the list for longer words to adjust difficulty.
///
/// # Arguments
///
/// * `words` - A slice of String objects to choose from, assumed to be sorted
///             from most common to least common.
/// * `word_length` - The length of the word we're looking for.
///
/// # Returns
///
/// An `Option<&str>` which is:
/// - `Some(&str)` containing a reference to a random word.
/// - `None` if the input slice is empty.
pub fn get_random_word(words: &[String], word_length: usize) -> Option<&str> {
    // We create a new variable `search_pool` that is a slice of the original `words` data.
    let search_pool = if word_length >= 5 {
        &words[0..1000.min(words.len())]
    } else if word_length == 4 {
        &words[0..700.min(words.len())]
    } else {
        &words[0..500.min(words.len())]
    };
    search_pool.choose(&mut rand::thread_rng()).map(String::as_str)
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