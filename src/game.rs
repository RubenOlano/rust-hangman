use std::io;
use std::io::prelude::*;

struct Game {
    word: String,
    guesses: i8,
    won: bool,
    lose: bool,
}

impl Game {
    fn init(word: &str) -> Game {
        Game {
            word: word.to_string(),
            guesses: 0,
            won: false,
            lose: false,
        }
    }

    fn read_guess(&mut self) {
        let mut guess = String::new();
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess).unwrap();
        guess = guess.trim().to_string();
        if guess.trim() == self.word {
            println!("The word was {}. You Win!", self.word);
            self.won = true;
        } else if self.guesses > 5 {
            self.lose = true;
            println!("Too many incorrect guesses. You Lose!")
        } else {
            self.guesses += 1;
            println!("{} was incorrect", guess,)
        }
    }
}

pub fn run() {
    let mut game = Game::init("Test");
    println!("Game Start!");
    while game.won != true && game.lose != true {
        game.read_guess();
    }
}
