use std::io;

struct Game {
    word: String,
    guesses: i8,
}

impl Game {
    fn init() -> Game {
        Game {
            word: "Test".to_string(),
            guesses: 0,
        }
    }

    fn read_guess(&mut self) {
        let mut guess = String::new();
        print!("Enter your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Nothing was received");
        if guess == self.word {
            println!("\nThe word was {}", self.word);
        } else {
            self.guesses += 1;
        }
    }
}

pub fn run() {
    let mut game = Game::init();
    game.read_guess();
}
