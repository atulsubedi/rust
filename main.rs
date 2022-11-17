// made clippy more sensitive to be sure the code is awesome
#![warn(clippy::all, clippy::pedantic)]
fn main(){
    let my_list = ["one","two","three","four"];
    for _num in &my_list {
        println!("{ }",_num);
    }
    for i in 0..4 {
        println!("{ }",i);
    } 
}
