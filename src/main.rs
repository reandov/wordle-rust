use rust_wordle::CurrentGame;
use std::io::stdin;

fn main() {
    let mut guess = String::new();

    let current_game = CurrentGame::new().unwrap();

    println!("Current game: {:?}", current_game);

    stdin()
        .read_line(&mut guess)
        .expect("Failed to read user input");

    // println!(
    //     "Is {:?} equal to {:?}: {:?}",
    //     &guess.trim(),
    //     current_game.selected_word,
    //     current_game.selected_word.eq(guess.trim())
    // );

    for iter in 0..5 {
        todo!()
        // CHECAR SE CHAR SLW[0] === GSS[0]
        // SE TRUE:
        // VALID_LETTERS.PUSH((LETTER, ITER))
        // SE FALSE:
        // SE A SLW.CONTAINS(GSS[0]):
        // VALID
    }
}
