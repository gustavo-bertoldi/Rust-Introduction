fn takes_ownership(_value: String) { }

fn does_not_take_ownership(_value: &String) { }

/**
 * This function appends to the string by taking ownership of the variable.
 * The ownership is given back to the main scope by returning the variable and re-assigning it to the original variable
 */
fn add_to_string(mut my_string: String, to_add: String) -> String {
  my_string.push_str(to_add.as_str());
  return my_string;
}

/**
 * This function appends to the string using a mutable reference to it.
 * We don't need to return anything because by using a reference we didn't take ownership of the variable.
 */
fn add_to_string_reference(my_string: &mut String, to_add: String) {
  my_string.push_str(to_add.as_str());
}

fn main() {
  let my_var = String::from("This is my String");

  //When we pass a variable to the function takes_ownership, its ownership is moved
  //takes_ownership(my_var);
  //So we cannot use it anymore as we don't have its ownership
  //println!("{}", my_var);
  
  //By using clone we are creating a copy of the value, so we don't move the ownership to the function
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

  //The function add_to_string takes ownership of the value but gives it back by returning it
  my_mut_var = add_to_string(my_mut_var, String::from(". Add this"));
  println!("{}", my_mut_var);

  //The function add_to_string_reference uses a reference to our variable, but as it mutates its value, we need to pass a mutable reference
  add_to_string_reference(&mut my_mut_var, String::from(". Now by reference."));
  println!("{}", my_mut_var);

}