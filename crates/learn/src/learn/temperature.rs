pub fn print_conversions() {
  let celsius = 0.0;
  let fahrenheit = 32.0;

  println!(
    "{:.1}°C = {:.1}°F",
    celsius,
    convert_celsius_to_fahrenheit(celsius)
  );
  println!(
    "{:.1}°F = {:.1}°C",
    fahrenheit,
    convert_fahrenheit_to_celsius(fahrenheit)
  );
}

fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
  celsius * 1.8 + 32.0
}

fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
  (fahrenheit - 32.0) / 1.8
}
