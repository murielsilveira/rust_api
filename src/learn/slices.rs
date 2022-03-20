pub fn print_randon_string_slices_info() {
  let a = String::from("abc🤮 efg");

  println!("{}", first_word(&a));

  for (i, c) in a.as_bytes().iter().enumerate() {
    println!("{} - {}", i, c);
  }

  // Taking a string slice in the middle of a utf-8
  // character compiles but fails at runtime.
  // let b = &a[..5];
  // println!("{}", b);
}

// also valid, but less flexible and idiomatic
// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
  for (i, &item) in s.as_bytes().iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}
