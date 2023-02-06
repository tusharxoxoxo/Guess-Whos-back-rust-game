# Guess-Who-s-Back-

The "Guess-Who's-Back" project is a simple guessing game written in the Rust programming language. The game uses a Command Line Interface (CLI) and includes various functions and associated methods. The game generates a random integer between 1 and 100 and prompts the player to enter a guess. If the guess is too low or too high, the program will indicate as such, and if the guess is correct, the game will print a congratulatory message and exit. The project is a work in progress and is being developed using external crates, with a focus on efficient and robust code. 

the program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.
Guess-Who-s-Back-
A simple guessing game in rust
This program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.
Processing a Guess
The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. To start, we'll allow the player to input a guess. Enter the code in Listing 2–1 into src/main.rs.
Filename: src/main.rs
use std::io;
fn main() {
    println!("Guess the number!");
println!("Please input your guess.");
let mut guess = String::new();
io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
println!("You guessed: {guess}");
}

github repo link = https://github.com/tusharxoxoxo/Guess-Who-s-Back-


work in progress
work in progress
work in progress
work in progress
work in progresswork in progress
work in progress

work in progress
work in progressv
v
