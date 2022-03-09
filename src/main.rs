use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  guessing_game();
}

fn guessing_game() {
  println!("This is a guessing game, you need to guess a number from 1 to 10!");

  let secret = rand::thread_rng().gen_range(1..11);
  println!("The secret number is {}", secret);

  loop {
    println!("Input your guess:");

    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a natural number!");

    // println!("Your guess is: {}", guess);
    match guess.cmp(&secret) {
      Ordering::Less => println!("To small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => println!("Nice guess! 🎊"),
    }
  }
}
