// using use keyword so that we don't have to type out the full version every time while taking
// input from the terminal
use std::io::stdin;
fn main(){
   println!("Hello, what's your name?");
   // mut lets you to change the value of variable 
   let mut your_name = String::new();
    // stdin() returns an object granting access to the Standard input
   stdin() 
       // read_line() is a method it receives input from keyboard
       .read_line(&mut your_name)
       .expect("Failed to read line");
   println!("Hello, { }",your_name)
}
