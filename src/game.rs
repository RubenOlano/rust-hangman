mod board;

use rand::Rng;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum GameErr {
    UnableToFetchWords(io::Error),
    UnableToReadWords(io::Error),
    UnableToFetchGuess,
    ErrorPushingWordToBoard,
}

struct Game {
    word: String,
    guesses: usize,
    won: bool,
    lose: bool,
    underscores: Vec<char>,
    prev_guesses: Vec<char>,
}

impl Game {
    fn read_file() -> Result<Vec<String>, GameErr> {
        match File::open("src/web2.txt") {
            Ok(mut f) => {
                let mut contents = String::new();
                match f.read_to_string(&mut contents) {
                    Ok(_) => Ok(contents.lines().map(|x| x.to_string()).collect()),
                    Err(e) => Err(GameErr::UnableToReadWords(e)),
                }
            }
            Err(e) => Err(GameErr::UnableToFetchWords(e)),
        }
    }

    fn init() -> Game {
        match Game::read_file() {
            Ok(words) => {
                let rand_index = rand::thread_rng().gen_range(0..words.len());
                let word = &words[rand_index];
                let underscores: Vec<char> = word.chars().map(|_| '_').collect();
                Game {
                    word: word.to_string(),
                    guesses: 0,
                    won: false,
                    lose: false,
                    underscores: underscores,
                    prev_guesses: Vec::new(),
                }
            }
            Err(e) => {
                println!("{:?}", e);
                Game {
                    word: "".to_string(),
                    guesses: 0,
                    won: false,
                    lose: false,
                    underscores: Vec::new(),
                    prev_guesses: Vec::new(),
                }
            }
        }
    }

    fn set_underscores(&mut self) {
        self.underscores = self.word.chars().map(|_| '_').collect();
    }

    fn read_guess(&mut self) -> Result<(), GameErr> {
        let mut guess = String::new();
        let underscores: String = self.underscores.iter().collect();
        let guessed: String = self.prev_guesses.iter().collect();
        println!("Word: {}", underscores);
        println!("Guesses: {}", guessed);
        let mut exit: bool = false;
        while !exit {
            print!("Enter your guess: ");
            io::stdout().flush().unwrap();
            match io::stdin().read_line(&mut guess) {
                Ok(_) => {
                    guess = guess.trim().to_string();
                    if guess.len() == 1 {
                        let guess_chars: char = match guess.chars().next() {
                            Some(c) => c,
                            None => return Err(GameErr::UnableToFetchGuess),
                        };
                        if self.prev_guesses.contains(&guess_chars) {
                            println!("You already guessed that already!");
                        } else {
                            exit = true;
                            let chars: Vec<char> = guess.chars().collect();
                            println!("You guessed: {}", chars[0]);
                            self.letters_in_word(chars[0])?;
                        }
                    } else {
                        println!("Please enter a single character!");
                    }
                }
                Err(e) => {
                    return Err(GameErr::UnableToReadWords(e));
                }
            }
        }
        Ok(())
    }

    fn letters_in_word(&mut self, guess: char) -> Result<(), GameErr> {
        let mut correct: bool = false;
        let mut underscores = vec![];
        let guess_lower: Vec<char> = guess.to_uppercase().collect();
        for a in 0..self.word.len() {
            let letter_lower: Vec<char> =
                self.word.chars().nth(a).unwrap().to_uppercase().collect();
            if letter_lower[0] == guess_lower[0] {
                if let Some(letter) = self.word.chars().nth(a) {
                    underscores.push(letter);
                    correct = true;
                } else {
                    return Err(GameErr::ErrorPushingWordToBoard);
                }
            } else if self.underscores[a] != '_' {
                if let Some(letter) = self.word.chars().nth(a) {
                    underscores.push(letter);
                } else {
                    return Err(GameErr::ErrorPushingWordToBoard);
                }
            } else {
                underscores.push('_');
            }
        }
        self.underscores = underscores;
        self.validate_guess(correct, guess);
        Ok(())
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

pub fn run() -> Result<(), GameErr> {
    let mut game = Game::init();
    game.set_underscores();
    println!("Game Start!");
    while !game.won && !game.lose {
        println!("\x1B[H\x1B[2J");
        board::get_state(game.guesses);
        game.read_guess()?;
    }
    board::get_state(game.guesses);
    Ok(())
}
