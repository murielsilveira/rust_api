use std::io;

fn main() {
  guessing_game();
}

fn guessing_game() {
  println!("This is a guessing game, you need to guess a number from 1 to 10!");

  println!("Input your guess:");

  let mut guess = String::new();
  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

  println!("Your guess is: {}", guess);
}
