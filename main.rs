fn main(){
    let my_list = ["one","two","three","four"];
    for num in &my_list {
        println!("{ }",num);
    }
    for i in 0..4 {
        println!("{ }",i);
    } 
}
