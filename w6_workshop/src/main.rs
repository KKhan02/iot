#[derive(Debug)]
struct User{
    name:String,
    roll_number:u32,
}

impl User{
    fn printing(& self){
        println!("Roll number is {}",self.roll_number);
        println!("Name is {}",self.name);
        println!("{:#?}",self);

    }
    fn welcome(name:String,roll_number:u32) -> User{
        println!("Welcome in Associated Function Biatch");
        User{
            name:name,
            roll_number:roll_number,
        }
    }
}


fn main() {
    // let age :u8 = 66;
    // println!("{:#?}",age);
    // let string_number = 44.to_string();
    // println!("{:#?}",string_number);
    // let string2num:u8 = string_number.parse().unwrap();
    // println!("{:#?}",string2num);
    // let mut input = String::new();
    // println!("Enter your age");
    // std::io::stdin().read_line(&mut input).expect("Error");
    // println!("You entered {:#?}",input);
    // let input_integer:u8 = input.trim().parse().unwrap();
    // println!("Converted Integer is {:#?}",input_integer);

//     let mut user_1 = User{
//         name:String::from("Khuzaim"),
//         roll_number:12345,
//     };
// //    user_1.printing();
//    let user_2 = User::welcome(String::from("Mrow"),12345000);
// //    user_2.printing();
//     User::welcome(String::from("Chubbubs"),12345000).printing();
//     let x = 12;
//     check(&x);

    let recieve = getdata();
    println!("{:#?}",recieve);
    let (temp,today) = getdata();
    println!("{},{}",temp,today);


}


fn check(x:&i32){
    let k = 10;
    println!("x:{:#?}, k:{:#?}",x,k);

    if &(x+k) >= x{
        println!("Sup");
    } 
}


fn getdata() ->(f32,String){
    let day = "Tuesday".to_string();
    let temperatue = 41.5;
    (temperatue,day)
}