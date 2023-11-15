use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn start() {
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
