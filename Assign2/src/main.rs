// fn main() {
//     let mut integer:u32 = 15;
//     let mut float:f32 = 2.55;
//     let mut string = String::from("Hello");
//     println!("Integer value: {},Float value: {},String value: {}",integer,float,string);

//    let power = changes(integer,float, &mut string);

//     println!("Returned the power of {} which is {}, also returned string {}",float,power,string);

// }

// fn changes(x:u32, y:f32, some_string: &mut String) -> f32{
//     for i in 0..x+1{
//         println!("{}",i);
//     }
//     some_string.push_str(" world");
//     f32::powi(y,2)    

// }

fn main(){
    let mut a = String::from("Hello world");
    let b = user(a);
    println!("{}",b);
    let a = 12;
    let b = 13;
    let c = a+b;
    println!("{}",c);
}

fn user(mut some_string:String) -> String {
//   println!("{}",some_string);
  some_string.push_str(" from user");
//   println!("{}",some_string);
  some_string


}