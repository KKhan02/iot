use std::io;
use rand::Rng;
#[derive(Debug)]
struct Food{
    restaurant: String,
    food_item: String,
    size:u8,
    price:u16,
    availability:bool,
}

impl Food{
    fn billing(&self,rider:String){

        println!("Rider id {}",rider);
        println!("Pizza price {}",self.price);
        println!("Pizza {:#?}",self);    
    }
    fn appearance (&self){
        println!("The {} looks great, has price and size {}, {} inches",self.food_item,self.price,self.size);
    }

    fn new(restaurant:String,food_item:String, price:u16, size: u8, availability:bool) -> Food{
        Food{
            food_item,
            price,
            size,
            restaurant,
            availability,
        }
    }


}

    // custom define datatype -> Person 
        //instance            -> Allan
            //Attribute       -> Age/Height/Weight
                //methods     -> talk/walk/work



fn main() {
    // let pizza = Food{
    //     restaurant:String::from("Pizza 363"),
    //     food_items:String::from("Chicken Fajita"),
    //     size:20,
    //     price:1200,
    //     availability:true,
    // };

    // let karhai = Food{
    //     restaurant:String::from("BBQ Tonaight"),
    //     food_items:String::from("Chicken ginger"),
    //     size:1,
    //     price:1500,
    //     availability:true,
    // };

    // println!("Pizza price {}",pizza.price);
    // println!("Pizza {:#?}",pizza);
    // println!("Karhai {:#?}",karhai);

    // printing(pizza);
    // println!("Pathan Hotel: {:#?}",pc());

    // let pizza = Food::new(String::from("Pizza 363"),String::from("Creamy Tikka"), 1200, 20,true);

    // pizza.billing("habib-ul-lah".to_string());

    // pizza.appearance();

    let secret_number = rand::thread_rng().gen_range(1,5001);

    println!("Your lucky number is {}",secret_number);
    println!("Enter a new data");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("ERROR");
    println!("{}",input.trim());


    // let number = 7;
    // match number{
    //     1 => {println!("You have One");}
    //     2 => {println!("You have Two");}
    //     3 => {println!("You have Three");}
    //     4 => {println!("You have Four");}
    //     5 => {println!("You have Five");}
    //     _ => {println!("Something else");}


    // };

}

fn printing(data:Food){
    println!("Pizza price {}",data.price);
    println!("Pizza {:#?}",data);    
}

fn pc() -> Food {
    let chai = Food{
        restaurant:String::from("Pathan"),
        food_item:String::from("Doodh Patti"),
        size:2,
        price:100,
        availability:true,
    };
    chai
}
