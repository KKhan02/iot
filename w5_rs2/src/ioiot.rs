use std::io;

pub fn take_input()->String{
    println!("Enter the value that you want");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Enter value");
    input
}