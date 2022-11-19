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
    // calling the function what is your name to store the result in name
    let name = what_is_your_name();
    println!("{:?}", name);
        
 if name  == "atul"{
    println!("Welcome");
} else {
    println!("Sorry, you are not on the list")
}
}
