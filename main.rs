fn main() {
  let name: String = String::from("Gustavo");
  hello(name);
}

fn hello(name: String) {
  let to_return = format!("Hello, {}", name);
  println!(format!("Hello, {}", name));
}