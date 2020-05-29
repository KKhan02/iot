use std::io;
mod ioiot;

fn main() {

    println!("Enter the value that you want:");
    let mut Khuzaim_IOT045022 = String::new();
    io::stdin().read_line(&mut Khuzaim_IOT045022);
    println!("The value is this: {}", Khuzaim_IOT045022);
    println!("Enter the first number:");

    let mut Khuzaim_IOT045022_01 = String::new();
    io::stdin().read_line(&mut Khuzaim_IOT045022_01);
    let Khuzaim_IOT045022_01: i32 = Khuzaim_IOT045022_01.trim().parse().unwrap();
    
    println!("Enter the second number:");

    let mut Khuzaim_IOT045022_02 = String::new();
    io::stdin().read_line(&mut Khuzaim_IOT045022_02);
    let Khuzaim_IOT045022_02: i32 = Khuzaim_IOT045022_02.trim().parse().unwrap();


    println!("The addition of {} and {} is {}",Khuzaim_IOT045022_01,Khuzaim_IOT045022_02,(Khuzaim_IOT045022_01+Khuzaim_IOT045022_02));

    // For FLoating Point Variables
    println!("Enter the first float number:");

    let mut Khuzaim_IOT045022_01 = String::new();
    io::stdin().read_line(&mut Khuzaim_IOT045022_01);
    let Khuzaim_IOT045022_01: f32 = Khuzaim_IOT045022_01.trim().parse().unwrap();
    
    println!("Enter the second float number:");

    let mut Khuzaim_IOT045022_02 = String::new();
    io::stdin().read_line(&mut Khuzaim_IOT045022_02);
    let Khuzaim_IOT045022_02: f32 = Khuzaim_IOT045022_02.trim().parse().unwrap();


    println!("The addition of {} and {} is {}",Khuzaim_IOT045022_01,Khuzaim_IOT045022_02,(Khuzaim_IOT045022_01+Khuzaim_IOT045022_02));

    let Khuzaim_IOT045022_01: f32 = ioiot::take_input().trim().parse().unwrap();
    let Khuzaim_IOT045022_02: f32 = ioiot::take_input().trim().parse().unwrap();
    println!("The addition of {} and {} is {}",Khuzaim_IOT045022_01,Khuzaim_IOT045022_02,(Khuzaim_IOT045022_01+Khuzaim_IOT045022_02));


}
