fn hello(name: String) -> String {
  return format!("Hello, {}", name);
}

fn square_root(number: f64) -> f64 {
  return number.sqrt();
}

fn main() {
  let my_var: String = String::from("World");
  println!("{}", hello(my_var));
  println!("{}", my_var);

  my_var = String::from("Team");
  println!("{}", hello(my_var));

  /*
  my_var: f32 = 9.0;
  println!("{}", square_root(my_float));

  my_var = 16.0;
  println!("{}", square_root(my_float));
  */
}