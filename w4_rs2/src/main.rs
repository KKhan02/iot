// fn main() {
//     // let mut counter = 0;
//     // while counter < 5{
//     //     println!("{}",counter);
//     //     counter +=1;
//     // }

//     // let array = [0,2,3,4,5,6,7];
//     // for index in array.iter(){
//     //     println!("{}",index);

//     // }
//     // for index in 0..8{
//     //     println!("{}",array[index]);
//     // }

//     // let (x,y) = account(1234);

//     // println!("{},{}",x,y)

//     let sameData = [34,33,45,23,45,221,324];

//     // let diffData = ("name",34,5.9);

//     // let hybridData = [("waseem",34,5.9),("faheem",34,5.9),("shahid",34,5.9)];

//     // println!("The value is {:?}",sameData);
//     // println!("The value is {:?}",diffData);
//     // println!("The value is {:?}",hybridData);

//     // println!("The value is {:?}",sameData[4]);
//     // println!("The value is {:?}",diffData.1);
//     // println!("The value is {}",hybridData[0].1);

//     println!("The max value is {}",largest(&sameData));

// }

// fn largest(arrNum: &[i32])->i32{
//     let mut max_num = arrNum[0];
//     for value_check in 1..arrNum.len(){

//         if max_num < arrNum[value_check]{
//             max_num = arrNum[value_check]

//         }

//     }

//     max_num

// }

// fn account(num:i32) -> (i64,i32){
//     let account_nunber:i64 = 1234455666;
//     let amount = 10_000;
//     if num == 1234{
//         return (account_nunber,amount);
//     }
//     else{
//         println!("not found");
//         return (0,0);
//     }
// }


// fn main(){
//     let sameData = [32,53,64,234,76];
//     println!("The lenth of the array is this: {}",sameData.len());
//     for index in 0..=4{
//         println!("The values are: {}",sameData[index]);
//     }
// }

// use std::env;

// fn main(){
//     let value_01 = env::args().nth(1);
//     let value_02 = env::args().nth(2);

//     println!("The value is {} {} ",value_01.unwrap(),value_02.unwrap());

// }

mod sub_function;

fn main(){
    sub_function::print_hello();
    let number = 5;
    println!("The square of the number {} is {}",number,sub_function::square(number));
}