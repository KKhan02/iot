fn main() {
    // let s1 = String::from("Hello Pakistan");
    // let result = length(&s1);
    // println!("The length of {} is {}",s1,result);
    // let mut s = String::from("Hello");

    // {let a = &mut s;
    // println!("{}",a);
    // a.push_str("world");
    // println!("{}",a);

    // }
    // {
    //     let b = &mut s;
    //     println!("{}",b);

    // }


    // change(&mut s);

    // let b = &s;
    // let c = &s;
    // let d = &s;
    // println!("{}",b);
    // println!("{}",c);
    // println!("{}",d);    
    
    // let a = &mut s;
    // println!("{}",a);
    let result = dangle();
    println!("{}",result);








}

// fn change(x: & mut String){
//    x.push_str("world");
//    println!("{}",x);

// }

// fn length(x:&String) -> usize{
//     x.len()
// }

fn dangle() -> String {
    let s = String::from("Hello");
    s
}