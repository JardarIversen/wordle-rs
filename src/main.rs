mod word_reader;
mod word_validation;

use word_reader::get_words_by_length;
use word_validation::get_random_word;

fn main() {
    let game_char_amount = 5;

    let possible_words = get_words_by_length(game_char_amount, "top-20k-words.txt")
        .expect("Failed to read word file. Make sure 'top-20k-words.txt' is in the root directory.");

    let word_to_guess_option = get_random_word(&possible_words);

    println!("Game characters: {}", game_char_amount);
    println!("Amount of possible words: {}", possible_words.len());

    match word_to_guess_option {
        Some(word) => {
            println!("Word to guess this game is: {}", word);
            println!(
                "This is word {}/{} for {} letters.",
                possible_words.iter().position(|w| w == word).unwrap(),
                possible_words.len() ,
                game_char_amount
            );
        }
        None => {
            println!("Could not select a word because the list of possible words is empty!");
        }
    }
}
