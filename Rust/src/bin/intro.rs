fn takes_ownership(_value: String) { }

fn does_not_take_ownership(_value: &String) { }

fn add_to_string(my_string: &mut String, to_add: String) {
  my_string.push_str(to_add.as_str());
}

fn main() {
  let my_var = String::from("This is my String");

  //When we pass a variable to the function takes_ownership, its ownership is moved
  //takes_ownership(my_var);
  //So we cannot use it anymore as we don't have its ownership
  //println!("{}", my_var);
  
  //By using clone we are creating a copy of the value, so we dont move the ownership to the function
  takes_ownership(my_var.clone());

  //Here we pass a reference to the value, not ownership
  //In Rust, we can have as much immutable references as we want at a time, but only one mutable
  does_not_take_ownership(&my_var);

  //The println! macro implements a mechanism so it doesn't take ownership of the variable
  println!("{}", my_var);

  //We cannot change the value of an immutable variable
  //my_var = String::from("This is my new String");

  let mut my_mut_var = String::from("This is my mutable variable");
  println!("{}", my_mut_var);

  //We can change its value because it was declared as mutable
  my_mut_var = String::from("This is my mutable String");
  println!("{}", my_mut_var);

  //The function add_to_string mutates its value, so we have to pass a mutable reference to it
  add_to_string(&mut my_mut_var, String::from(". It's awesome."));
  println!("{}", my_mut_var);

}