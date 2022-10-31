use std::io;

fn main() {
    //println!("Hello, world!");
    println!("Guess-Who-s-Back-🧐🤨🤔💭💭💭");
    println!("Input your guessss");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line😫😖😣");
    println!("You guessed: {guess}");
    
}
