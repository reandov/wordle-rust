use rust_wordle;
use std::io::stdin;

fn main() {
    let mut guess = String::new();
    let selected_word = rust_wordle::select_random_word();

    println!("Selected word: {}", selected_word);

    stdin()
        .read_line(&mut guess)
        .expect("Failed to read user input");

    println!(
        "Your guess is: {}\nThe word is: {}\nIt's equal? {:?}",
        guess,
        selected_word,
        (selected_word.eq(&guess.trim()))
    )
}
