use rust_wordle::CurrentGame;
use std::io::stdin;

fn main() {
    let mut guess = String::new();

    let mut current_game = CurrentGame::new().unwrap();

    println!("Current game: {:?}", current_game);

    stdin()
        .read_line(&mut guess)
        .expect("Failed to read user input");

    for iter in 0..5 {
        let original_char = current_game.selected_word.chars().nth(iter).unwrap();
        let guess_char = guess.chars().nth(iter).unwrap();

        if original_char == guess_char {
            current_game.valid_letters.push((guess_char, iter, true))
        } else if original_char != guess_char {
            if current_game.selected_word.contains(guess_char) {
                current_game.valid_letters.push((guess_char, iter, false))
            } else {
                current_game.invalid_letters.push(guess_char);
            }
        }
    }

    println!("Current game: {:?}", current_game);
}
