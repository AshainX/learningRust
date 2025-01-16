mod randomno; // Importiing the randomno module

fn main() {
    let secret_number = randomno::generate_number(1, 1000); //generate a rand no
    randomno::play_guessing_game(secret_number);
}