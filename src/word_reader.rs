use std::fs::File;
use std::io::{self, BufRead, BufReader};
// use std::path::Path;

/// Reads a file of newline-separated words and returns a vector of words
/// that match the specified length.
///
/// # Arguments
///
/// * `length` - The desired length of the words to filter by.
/// * `path` - The path to the word list file.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(Vec<String>)` containing the list of words if successful.
/// - `Err(io::Error)` if the file cannot be opened or read.
pub fn get_words_by_length(length: usize, path: &str) -> io::Result<Vec<String>> {
    // 1. Open the file specified by the path.
    let file = File::open(path)?;

    // 2. Create a buffered reader for efficiency.
    let reader = BufReader::new(file);

    // 3. Process the lines of the file.
    let words = reader
        .lines() // Get an iterator over the lines. Each item is an io::Result<String>
        .filter_map(Result::ok) // Discard any lines that failed to read, and unwrap the good ones.
        .filter(|line| line.trim().len() == length) // Keep only lines where the trimmed length matches.
        .collect(); // Collect the filtered lines into a Vec<String>.

    Ok(words)
}

#[cfg(test)]
mod tests {
    // Import the function we want to test from the parent module.

    use super::*;

    #[test]
    fn test_get_5_letter_words() {
        // Create a dummy file for this test.
        let file_content = "rust\ncrane\nhouse\nwordle\napple\n";
        let test_file_path = "test_words.txt";
        std::fs::write(test_file_path, file_content).expect("Unable to write test file");

        // Call the function with a length of 5.
        // We use .unwrap() in tests because if this fails, the test should fail anyway.
        let five_letter_words = get_words_by_length(5, test_file_path).unwrap();

        // Check that the result is what we expect.
        assert_eq!(five_letter_words, vec!["crane", "house", "apple"]);
        
        // Clean up the dummy file.
        std::fs::remove_file(test_file_path).expect("Unable to remove test file");
    }

    #[test]
    fn test_file_not_found() {
        // Call the function with a path that we know doesn't exist.
        let result = get_words_by_length(5, "non_existent_file.txt");

        // Assert that the function returned an error.
        assert!(result.is_err());
    }

    #[test]
    fn analyze_word_counts_in_real_file() {
        let path = "top-20k-words.txt";
        let length_to_test = 12; // The classic Wordle length

        // We expect this to work, so we'll use .expect() to get the result.
        // If this panics, the test will fail, which is what we want.
        let words = get_words_by_length(length_to_test, path)
            .expect("Should be able to read the real top-20k-words.txt file");

        // The test runner normally hides output from println!. We'll run a special
        // command to see this. This is great for debugging and analysis.
        println!(
            "Analysis: Found {} words of length {} in '{}'.",
            words.len(),
            length_to_test,
            path
        );

        // A good basic test is to ensure we got *some* words.
        // This assertion will fail if the `words` vector is empty.
        assert!(!words.is_empty());

        // Once you run this and see the count, you could make the test
        // more specific, for example:
        // assert_eq!(words.len(), 1546); // (This number is just an example)
    }
}