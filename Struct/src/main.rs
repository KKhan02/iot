#[derive(Debug)]

struct Book{
    name: String,
    author: String,
    price: u16,
    availability: bool,

} //template


struct Color (i32,i32,i32); 
struct Points (i32,i32,i32);




fn main() {

    // let mut book_1 = Book{
    //     name:String::from("Book A"),
    //     author: String::from("Author A"),
    //     price:500,
    //     availability:true,

    // };
    // let book_2 = Book{
    //     name:String::from("Book B"),
    //     author: String::from("Author B"),
    //     price:book_1.price,
    //     availability:book_1.availability,

    // };

    // println!("{:#?}",book_2);

    let black = Color(6,9,0);
    println!("{}",black);
    // another_function(black);








    // let book_1 = build(String::from("Book A"),String::from("Author A"));
    //                                 //name                  //author
    // println!("{:#?}", book_1);


}

// fn build(name:String,author:String) ->Book {
// // we could also have defined another variable to store and then pass this
// // but we directly declared and returned the value.
//     Book{ 
//         //key:parameter_value
//         name,     
//         author,
//         price:500,
//         availability:true,
//     } // if the key or field and function parameters both have same name
//     // we donot write them two times, we just write name and author once.

// }


// fn another_function(x:Color){
//     println!(":#?",x);
// }