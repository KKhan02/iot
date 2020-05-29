fn main() {
    // let mut count = 1;
    // let value = loop{
    //     println!("{}. Hello World",count);
    //     if count ==10 {
    //         break count
    //     }
    //     count = count + 1;
    // } ;
    // println!("{}",value);

    // let mut counter = 0;
    // while counter < 5{
        // println!("Sup peeps!!");
        // counter = counter+1;
    // }
// 
    // let mut counter_1 = 0;
    // let array = [10,14,25,67,100,98];
// 
    // while counter_1 < array.len(){
        // println!("{}",array[counter_1]);
        // counter_1 = counter_1 + 1;
    // }

    for a in (0..5).rev(){
        println!("{}. Hello world!!",a);
    }

    let array = [10,14,25,67,100,98];
    for num in array.iter(){
        println!("{}",num);
    }

}

