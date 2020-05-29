// fn main() {
//     let data1 = 15;

//     let data2 = data1;

//     let string_data1 = "hello";

//     let string_data2 = string_data1;

//     println!("data1: {}, data2: {}",data1,data2);
//     println!("string_data1: {}, string_data2: {}",string_data1,string_data2);

//     let x = String::from("hello piaic");

//     println!("x: {}",x);

//     let y = x;

//     println!("y: {}",y);


// }


// fn main(){
    // let age1 :u8 = 22;
    // println!("age is {}",age1);

    // let food = String::from("Zinger Burger");
    // println!("Food is {}",food);
    // println!("Food is {:?}",food);
    // kfc(food);

    // let mut chai = String::new();
    // chai.push_str("Doodh patti");
    // println!("Data of chai {}",chai);
    // println!("Pointer/Adress/Reference {:p}",&chai);
    // println!("Length of chai {}",chai.len());
    // pc(&chai);
    // println!("After sending to function");
    // println!("Data of chai {}",chai);
    // println!("Pointer/Adress/Reference {:p}",&chai);
    // println!("Length of chai {}",chai.len());

//     let mut food3 = String::new();
//     println!("{:?}",food3);
//     tandoadam(&mut food3);
//     println!("{:?}",food3);
// }

// fn kfc(deal:String){
//     println!("We are in KFC");
//     println!("Today deal is {}",deal);

// }

// fn pc(tea : &String){
//     println!("We are in PC");

// }

// fn tandoadam(today:&mut String){
//     today.push_str("Mutton Sajji");
//     println!("After push {}",today);
// }

fn main(){
    // let aftari = ["Dahi bare", "Samosay", "Pakoray","Chaat"];

    // for mydata in aftari.iter(){
    //     println!("{}",mydata);
    // }

    // for num in 0..=10{
    //     println!("{}",num);

    // }
    // for num in (0..=10).rev(){
    //     println!("{}",num);

    // }

    // for num1 in (0..=5){
    //     println!("{:?}",aftari.get(num1));
    // }

    let game = "Cricket and Football";
    let game1 = &game[0..7];
    println!("Game1 {}",game1);
    let game2 = &game[12..=19];
    println!("Game1 {}",game2);

    let mut score: u8 = 0;
    loop{
        println!("Your score is {}",score);
        score +=1;
        for delay in 0..100000{

        }
    }
    println!("End of Program");

}