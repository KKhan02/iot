use std::io;

fn main() {
    let mut s = String::new(); // ::new() is an associated function which creates a new empty string
    // The line has created a mutable variable that is currently bound to a new, empty instance of a string
    io::stdin().read_line(&mut s)
        .expect("Failed to read a line");
// stdin is a method that will return an object that will point towards 
//whatever we are typing on the console.
// then by using readline we are telling rust to put whatever we are typing in the console in 
//the variable s. if an error comes, the line in expect method will execute.
println!("{}",s);

    let mut s_integer:u32 = s.trim().parse()
        .expect("Please enter a digit");
    s_integer = s_integer + 1;
    println!("{}",s_integer)

}
