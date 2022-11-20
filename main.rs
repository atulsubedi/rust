use std::io::stdin;
// -> String denotes that it returns a string
 fn what_is_your_name() -> String{
     let mut your_name = String::new();
     stdin()
         .read_line(&mut your_name)
         .expect("Failed to read line");
     your_name
         .to_lowercase()
         .trim()
 }
fn main(){
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("{}",name);

    let visitor_list : [&str;2] = ["atul","hl"];

    // rust for loop operates on ranges rather than a set of numbers
     let mut allow_them_in = false;
    for i in visitor_list.len(){
        if visitor == &name{
            allow_them_in = true;
        }
    } 
if allow_them_in {
    println!("Welcome to tree house, {}",name);
} else{
    println!("Sorry, only hl allowed");
}
}



