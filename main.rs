use std::io::stdin;
// -> String denotes that it returns a string
 fn what_is_your_name() -> String{
     let mut your_name = String::new();
     stdin()
         .read_line(&mut your_name)
         .expect("Failed to read line");
     your_name
         .trim()
         .to_lowercase()

 }
fn main(){
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("{}",name);

    let visitor_list = ["atul","hl"];

    // rust for loop operates on ranges rather than a set of numbers
     let mut allow_them_in = false;
    for visitor in &visitor_list {
        if visitor == &name{
            allow_them_in = true;
        }
    } 
    // check if your name is in the list to enter in the house
if allow_them_in {
    println!("Welcome to tree house, {}",name);
} else{
    println!("Sorry, Person on the list are only allowed");
}
}



