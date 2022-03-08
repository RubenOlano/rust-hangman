mod board;

use rand::Rng;
use std::io;
use std::io::prelude::*;

struct Game {
    word: String,
    guesses: usize,
    won: bool,
    lose: bool,
    underscores: String,
}

impl Game {
    fn init() -> Game {
        let words: [&str; 3] = ["Hi", "Bye", "No"];
        let rand_index: usize = rand::thread_rng().gen_range(0..3);
        let word = words[rand_index];
        Game {
            word: word.to_string(),
            guesses: 0,
            won: false,
            lose: false,
            underscores: "".to_string(),
        }
    }

    fn set_underscores(&mut self) {
        let mut underscores = String::new();
        for _a in self.word.chars() {
            underscores.push('_');
        }
        self.underscores = underscores;
    }

    fn read_guess(&mut self) {
        let mut guess = String::new();
        println!("{}", self.underscores);
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess).unwrap();
        self.validate_guess(guess.trim())
    }

    fn validate_guess(&mut self, guess: &str) {
        if guess.to_lowercase() == self.word.to_lowercase() {
            println!("The word was {}. You Win!", self.word);
            self.won = true;
        } else if self.guesses > 6 {
            self.lose = true;
            println!(
                "Too many incorrect guesses. You Lose! The word was {}",
                self.word
            )
        } else {
            self.guesses += 1;
            println!("{} was incorrect", guess)
        }
    }
}

pub fn run() {
    let mut game = Game::init();
    game.set_underscores();
    println!("Game Start!");
    while !game.won && !game.lose {
        board::get_state(game.guesses);
        game.read_guess();
    }
}
