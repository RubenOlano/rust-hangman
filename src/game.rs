mod board;

use rand::Rng;
use std::io;
use std::io::prelude::*;

struct Game {
    word: String,
    guesses: usize,
    won: bool,
    lose: bool,
    underscores: Vec<char>,
    prev_guesses: Vec<char>,
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
            underscores: vec![],
            prev_guesses: vec![],
        }
    }

    fn set_underscores(&mut self) {
        let mut underscores = vec![];
        for _a in self.word.chars() {
            underscores.push('_');
        }
        self.underscores = underscores;
    }

    fn read_guess(&mut self) {
        let mut guess = String::new();
        let underscores: String = self.underscores.iter().collect();
        let guessed: String = self.prev_guesses.iter().collect();
        println!("Word: {}", underscores);
        println!("Guesses: {}", guessed);
        let mut exit: bool = false;
        while !exit {
            print!("Enter your guess: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut guess).unwrap();
            guess = guess.trim().to_string();
            if guess.trim().len() == 1 {
                exit = true;
                let chars: Vec<char> = guess.chars().collect();
                println!("You guessed: {}", chars[0]);
                self.letters_in_word(chars[0])
            } else {
                println!("Guess must be one character. Try Again");
                guess = String::new();
            }
        }
    }

    fn letters_in_word(&mut self, guess: char) {
        let mut correct: bool = false;
        let mut underscores = vec![];
        let guess_lower: Vec<char> = guess.to_uppercase().collect();
        for a in 0..self.word.len() {
            let letter_lower: Vec<char> =
                self.word.chars().nth(a).unwrap().to_uppercase().collect();
            if letter_lower[0] == guess_lower[0] {
                underscores.push(self.word.chars().nth(a).unwrap());
                correct = true;
            } else if self.underscores[a] != '_' {
                underscores.push(self.word.chars().nth(a).unwrap())
            } else {
                underscores.push('_');
            }
        }
        self.underscores = underscores;
        self.validate_guess(correct, guess)
    }
    fn validate_guess(&mut self, correct: bool, guess: char) {
        if !correct {
            self.guesses += 1;
            self.prev_guesses.push(guess);
        }
        let underscores: String = self.underscores.iter().collect();
        if underscores == self.word {
            self.won = true;
            println!("\x1B[H\x1B[2J");
            println!("You win! The word was {}", self.word);
        }
        if self.guesses > 5 {
            self.lose = true;
            println!("Too many incorrect guesses! The word was {}", self.word);
        }
    }
}

pub fn run() {
    let mut game = Game::init();
    game.set_underscores();
    println!("Game Start!");
    while !game.won && !game.lose {
        println!("\x1B[H\x1B[2J");
        board::get_state(game.guesses);
        game.read_guess();
    }
    board::get_state(game.guesses);
}
