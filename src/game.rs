mod board;

use rand::Rng;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum GameErr {
    UnableToFetchWords(io::Error),
    UnableToReadWords(io::Error),
    ErrorReadingInput(io::Error),
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
    fn init() -> Self {
        // use the read_file function to get a vector of words
        match Game::read_file() {
            Ok(words) => {
                // pick a random word from the vector
                let rand_index = rand::thread_rng().gen_range(0..words.len());
                let word = &words[rand_index];
                // create a vector of underscores the same length as the word
                // create a new game with the word, guesses, and underscores
                Game {
                    word: word.to_string(),
                    guesses: 0,
                    won: false,
                    lose: false,
                    underscores: vec!['_'; word.len()],
                    prev_guesses: Vec::new(),
                }
            }
            Err(_) => {
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
    fn read_file() -> Result<Vec<String>, GameErr> {
        // Open the file and throw an error if it fails
        match File::open("src/web2.txt") {
            Ok(mut f) => {
                let mut contents = String::new();
                // Read the file into a string and throw an error if it fails
                match f.read_to_string(&mut contents) {
                    Ok(_) => Ok(contents.lines().map(|x| x.to_string()).collect()),
                    Err(e) => Err(GameErr::UnableToReadWords(e)),
                }
            }
            Err(e) => Err(GameErr::UnableToFetchWords(e)),
        }
    }

    fn read_guess(&mut self) -> Result<(), GameErr> {
        // Get the underscores as a string to print
        let underscores: String = self.underscores.iter().collect();
        // Get the user's previous guesses as a string to print
        let guessed: String = self.prev_guesses.iter().collect();

        println!("Word: {}", underscores);
        println!("Guesses: {}", guessed);

        // Loop to read user's guesses with "exit" as a flag to exit the loop
        let mut exit: bool = false;
        while !exit {
            // Read the user's guess
            self.user_guess(&mut exit)?;
        }
        Ok(())
    }

    fn user_guess(&mut self, exit: &mut bool) -> Result<(), GameErr> {
        // Mutable string to hold the user's guess
        let mut guess: String = String::new();
        print!("Enter your guess: ");
        // Flush output
        match io::stdout().flush() {
            Ok(_) => {}
            Err(e) => return Err(GameErr::ErrorReadingInput(e)),
        };

        // Read the user's guess
        Ok(match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                // trim any whitespace from the guess
                guess = guess.trim().to_string();
                //Make sure the guess is only one character long
                if guess.len() == 1 {
                    // Return single character guess to the game
                    let guess_chars: char = match guess.chars().next() {
                        Some(c) => c,
                        None => return Err(GameErr::UnableToFetchGuess),
                    };
                    // Check if duplicate guess
                    if self.prev_guesses.contains(&guess_chars) {
                        println!("You already guessed that!");
                    } else {
                        // Add the guess to the list of previous guesses
                        *exit = true;
                        let chars: Vec<char> = guess.chars().collect();
                        println!("You guessed: {}", chars[0]);
                        // Check if the guess is in the word
                        self.letters_in_word(chars[0])?;
                    }
                } else {
                    println!("Please enter a single character!");
                }
            }
            Err(e) => {
                return Err(GameErr::UnableToReadWords(e));
            }
        })
    }

    fn letters_in_word(&mut self, guess: char) -> Result<(), GameErr> {
        // vector of underscores
        let mut underscores = vec![];
        let guess_lower: Vec<char> = guess.to_uppercase().collect();
        let correct = match self.check_letters(&mut underscores, guess_lower) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        self.underscores = underscores;
        self.validate_guess(correct, guess);
        Ok(())
    }

    fn check_letters(&mut self, underscores: &mut Vec<char>, guess_lower: Vec<char>) -> Result<bool, GameErr> {
        let mut correct: bool = false;
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
        Ok(correct)
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
    println!("Game Start!");
    while !game.won && !game.lose {
        println!("\x1B[H\x1B[2J");
        board::get_state(game.guesses);
        game.read_guess()?;
    }
    board::get_state(game.guesses);
    Ok(())
}


