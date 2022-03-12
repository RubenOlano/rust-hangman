const BOARD_STATE: [&str; 7] = [
    "
    +---+
        |
        |
        |
       ===
        ",
    "
    +---+
    o   |
        |
        |
       ===
       ",
    "
    +---+
    o   |
    |   |
        |
       ===
        ",
    "
    +---+
    o   |
   /|   |
        |
       ===
        ",
    "
    +---+
    o   |
   /|\\  |
        |
       ===
        ",
    "
    +---+
    o   |
   /|\\  |
   /    |
       ===
       ",
    "
    +---+
    o   |
   /|\\  |
   / \\  |
       ===
    ",
];

pub fn get_state(num: usize) {
    match num {
        0..=7 => {
            println!("{}", BOARD_STATE[num].to_string())
        }
        _ => println!("How tf did you get here?"),
    }
}
