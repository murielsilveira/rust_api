pub fn print_first_ten() {
  for n in 0..10 {
    print!("{} ", nth_fibonacci(n));
  }
  println!();
}

pub fn nth_fibonacci(n: u32) -> u32 {
  if n == 0 {
    return 0;
  }
  if n <= 2 {
    return 1;
  }
  nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
}
