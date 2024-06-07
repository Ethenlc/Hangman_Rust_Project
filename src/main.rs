use std::io::{self, Write};
use std::collections::HashSet;
use rand::seq::SliceRandom;

fn main() {
    let words = vec!["temple", "programming", "moroni", "rust", "mountain", "shoes"];
    let secret_word = choose_random_word(&words);
    let mut guessed_letters = HashSet::new();
    let mut remaining_attempts = 6;

    println!("Welcome to Hangman!");

    while remaining_attempts > 0 {
        display_game_state(&secret_word, &guessed_letters, remaining_attempts);
        display_hangman(remaining_attempts);

        let guess = get_user_guess();
        
        if guessed_letters.contains(&guess) {
            println!("You already guessed '{}'", guess);
        } else if secret_word.contains(guess) {
            println!("Good guess!");
            guessed_letters.insert(guess);
        } else {
            println!("Wrong guess!");
            remaining_attempts -= 1;
            guessed_letters.insert(guess);
        }

        if is_word_guessed(&secret_word, &guessed_letters) {
            println!("Congratulations! You guessed the word '{}'", secret_word);
            return;
        }
    }

    display_hangman(remaining_attempts);
    println!("Game over! The word was '{}'", secret_word);
}

fn choose_random_word(words: &[&str]) -> String {
    let mut rng = rand::thread_rng();
    let word = words.choose(&mut rng).expect("Word list is empty");
    word.to_string()
}

fn display_game_state(secret_word: &str, guessed_letters: &HashSet<char>, remaining_attempts: usize) {
    let display: String = secret_word.chars()
        .map(|c| if guessed_letters.contains(&c) { c } else { '_' })
        .collect();

    println!("Word: {}", display);
    println!("Remaining attempts: {}", remaining_attempts);
    print!("Guessed letters: ");
    for letter in guessed_letters {
        print!("{} ", letter);
    }
    println!("\n");
}

fn get_user_guess() -> char {
    print!("Enter a letter: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().chars().next().expect("No input provided")
}

fn is_word_guessed(secret_word: &str, guessed_letters: &HashSet<char>) -> bool {
    for c in secret_word.chars() {
        if !guessed_letters.contains(&c) {
            return false;
        }
    }
    true
}

fn display_hangman(remaining_attempts: usize) {
    let hangman_states = [
        "
  ____
 |    |
 |    O
 |   /|\\
 |   / \\
 |
_|_____
",
        "
  ____
 |    |
 |    O
 |   /|\\
 |   / 
 |
_|_____
",
        "
  ____
 |    |
 |    O
 |   /|\\
 |    
 |
_|_____
",
        "
  ____
 |    |
 |    O
 |   /|
 |    
 |
_|_____
",
        "
  ____
 |    |
 |    O
 |    |
 |    
 |
_|_____
",
        "
  ____
 |    |
 |    O
 |    
 |    
 |
_|_____
",
        "
  ____
 |    |
 |    
 |    
 |    
 |
_|_____
"
    ];
    println!("{}", hangman_states[0 + remaining_attempts]);
}
