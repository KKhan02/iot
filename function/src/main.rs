fn main() {
    paper();
    let (number1,number2) = (2,8);
   let (x,y) = square(number1,number2);
    println!("The square of the number {} is {}",number1,x);
    println!("The square of the number {} is {}",number2,y)
    
}

fn paper(){

    println!("1. Add milk");
println!("2. Add Butter");
println!("3. Add eggs");
println!("4. Add sugar");
println!("5. Stir it");
    println!("6. Heat on gentle flame");
}

fn square(x:u32,y:u32) -> (u32,u32){
    let result = x*x;
    let result_1 = y*y;
    // println!("The square of the number {} is {}",x,result);
    // println!("The square of the number {} is {}",y,result_1)
    (result,result_1)

}




