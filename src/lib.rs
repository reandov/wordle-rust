use rand::{Error, Rng};

const WORDS: [&str; 30] = [
    "salto", "calça", "queda", "amora", "areio", "miúdo", "noite", "moita", "canal", "lento",
    "bolsa", "novos", "ósseo", "limpa", "peixe", "vamos", "vazou", "iscas", "houve", "jarro",
    "sinto", "gázua", "feixe", "trens", "arara", "chama", "sinas", "lazer", "roseo", "resma",
];

#[derive(Debug)]
pub struct CurrentGame {
    pub selected_word: String,
    pub tried_words: Vec<&'static str>,
    pub remaining_trials: i8,
    pub is_completed: bool,
    pub valid_letters: Vec<char>,
    pub invalid_letters: Vec<char>,
}

impl CurrentGame {
    pub fn new() -> Result<CurrentGame, Error> {
        Ok(CurrentGame {
            selected_word: select_random_word(),
            tried_words: Vec::from([]),
            remaining_trials: 5,
            invalid_letters: Vec::from([]),
            valid_letters: Vec::from([]),
            is_completed: false,
        })
    }
}

pub fn select_random_word() -> String {
    let random_number = rand::thread_rng().gen_range(0..WORDS.len());

    WORDS[(random_number)].to_string()
}
