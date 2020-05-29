fn main() {
    // for index in 1..6{
    //     println!("Hello, world!");
    // }
    // let mut index = 0;
    // loop{
    //     println!("{}. Hello, world!",index);
    //     index +=1;
    //     if index == 6 {
    //         break;
    //     }
    // }

    // while index < 6 {
    //         println!("{}. Hello, world!",index);
    //         index += 1;

    // }
    // let price_1 = 300;
    // println!("Value of price_1 is {}",price_1);
    // println!("Address of price_1 is {:p}",&price_1);
    // let price_1 = price_1 +100;
    // println!("Value of price_1 is {}",price_1);
    // println!("Address of price_1 is {:p}",&price_1);

    // let mut salaty:u16 = 10_000;
    // println!("Value of salary_1 is {}",salaty);
    // println!("Address of salary_1 is {:p}",&salaty);

    // salaty = salaty+1;
    // println!("Value of salary_1 is {}",salaty);
    // println!("Address of salary_1 is {:p}",&salaty);
    // let course = "IOT and AIC";
    // println!("{}",course);
    // let course = course.len();
    // println!("{}",course);


    let fruit = ["Melon","Water Melon", "Chiko"];
    let price = [60,12,45,10];
    let fruit_1 = ("Melon", "Yellow", 12,11);

    // println!("{:#?}",fruit);
    // println!("{:#?}",price);
    // println!("{:#?}",fruit_1);
    // println!("{}",fruit_1.2);
    // println!("{}",fruit[2]);

    for myfruit in fruit.iter(){
        print!("My fruit is {} ",myfruit);
    }

    let myfruitlength = fruit.len();
    println!("I have {} fruits",myfruitlength);




}
