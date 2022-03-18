#![allow(dead_code)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  // guessing_game();
  for n in 0..9 {
    print!("{} ", nth_fibonacci(n));
  }
}

fn guessing_game() {
  println!("This is a guessing game, you need to guess a number from 1 to 10!");

  let secret = rand::thread_rng().gen_range(1..11);
  println!("The secret number is {} 🤫", secret);

  'game: loop {
    println!("Type a natural number and press enter:");

    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
      Ok(number) => number,
      Err(_) => continue 'game,
    };

    match guess.cmp(&secret) {
      Ordering::Less => println!("To small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => {
        println!("Nice guess! 🎊");
        break 'game;
      },
    }
  }
}

fn nth_fibonacci(n: u32) -> u32 {
  if n == 0 {
    return 0;
  }
  if n <= 2 {
    return 1;
  }
  return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
}
