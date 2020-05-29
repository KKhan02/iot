// #[derive(Debug)]
// struct Rect{
//     height:u32,
//     width:u32,
// }

// fn main() {
//     // let width = 30;
//     // let height = 50;
//     // println!("The area of rectangle is {}",area(height,width));
//     // let rect_1 = (100,50);
//     // println!("The area of rectangle is {}",area_tuple(rect_1));
//     let rect_1 = Rect{
//         height:100,
//         width:50,
//     };
//     println!("The area of rectangle is {}",area_struct(&rect_1));
//     println!("{:#?}",rect_1)



// }

// fn area(height_1:u32,width_1:u32) -> u32 {
    
//     height_1*width_1

// }

// fn area_tuple(dimensions : (u32,u32)) -> u32{
//     dimensions.0 *dimensions.1
// }

// fn area_struct(rec:&Rect) ->u32{
//     rec.height * rec.width
// }


///user defined data type
struct Rectangle {
    height:u32,
    width:u32,
    }
    /////function
    impl Rectangle{
        fn area(&self) ->u32{
            self.width*self.height
        }
    }
    //main function
    fn main(){
        //instence/object_1
        let rec_1=Rectangle{
            height:10,
            width:20
        };
    //instence/object_2
    let rec_2=Rectangle{
        height:34,
        width:45,
    };
    let result=rec_1.area();
    let result_1=rec_2.area();
    println!("The area of rectangle object 1 is: {}",result);
    println!("The area of rectangle object_2 is :{}",result_1);
    }