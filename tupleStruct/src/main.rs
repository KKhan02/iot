#[derive(Debug)]
struct Color(i32,i32,i32);
struct Points(i32,i32,i32);



fn main() {
    let black = Color(6,9,0);
    // println!("{:#?}",black);
    another_function(black)
    let axis = Points(1,0,1);
}

fn another_function(x:Color){
    println!("{:#?}",x);
}