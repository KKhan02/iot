#[derive(Debug)]
struct Rectangle {
    height:u32,
    width:u32,
}

impl Rectangle {
    fn area (&self) -> u32{
        self.height * self.width
    }
    fn can_hold(&self,other: &Rectangle) -> bool{
        self.height > other.height && self.width > other.width

    }
    fn square (size:u32) ->Rectangle{
        Rectangle{
            width:size,
            height:size,
        }
    }
}



fn main() {
    // let rec_1 = Rectangle{
    //     height:100,width:50,
    // };
    // let rec_2 = Rectangle{
    //     height:90,width:10,
    // };
    // let result = rec_1.area();
    // let result_1 = rec_2.area();
    // println!("The area of Rectangle is {}", result);
    // println!("The area of Rectangle is {}", result_1);

    // let rec_1 = Rectangle{height:100,width:50,};
    // let rec_2 = Rectangle{height:90,width:40,};
    // let rec_3 = Rectangle{height:70,width:30,};

    // let result = rec_1.can_hold(&rec_2); // can rec1 hold rec2
    // println!("Rec_1 can hold Rec_2 : {}",result);
    // println!("Rec_1 can hold Rec_3 : {}",rec_1.can_hold(&rec_3));
    let result = Rectangle::square(8);
    println!("{:#?}",result)


} 
