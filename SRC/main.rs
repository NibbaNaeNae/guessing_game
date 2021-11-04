use std::io

fn main(){
    println!("Guess the number!");

    println!("Please enter your guess");

    let mut = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");
    
    println!("You guessed: {}", guess);
}

