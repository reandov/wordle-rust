use rand::Rng;

const WORDS: [&str; 30] = [
    "salto", "calça", "queda", "amora", "areio", "miúdo", "noite", "moita", "canal", "lento",
    "bolsa", "novos", "ósseo", "limpa", "peixe", "vamos", "vazou", "iscas", "houve", "jarro",
    "sinto", "gázua", "feixe", "trens", "arara", "chama", "sinas", "lazer", "roseo", "resma",
];

pub fn select_random_word() -> String {
    let random_number = rand::thread_rng().gen_range(0..WORDS.len());

    WORDS[(random_number)].to_string()
}
