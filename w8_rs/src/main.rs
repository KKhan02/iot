#[derive(Debug)]
enum Move{
    Walk, //variants
    Jump,//variants
    Hop,//variants
    Run,//variants
}

#[derive(Debug)]

enum Sweet{
    Chocolate (String),
    Sweet(String),
    HomeMad(String),
    Open(String,u32),
}
impl Move{

    // method in implementation block and has self parameters
    fn ppt(&self){
        println!("From implementation block {:#?}",self);
       
    }
}

fn daily(myvalue:Move){
    println!("{:#?}",myvalue);
    match myvalue{
        Move::Walk => println!("Noice"),
        Move::Jump => println!("Uchal kood krw"),
        Move::Hop => println!("Choote bachon ke jump"),
        Move::Run => println!("Bhaag Milka Bhaag"),
    };


}

fn increment(num:Option<i32>) -> Option<i32>{
    match num {
        None => None,
        Some(i) => Some(i+1),
    }
}


fn main() {
    let mymove = Move::Walk;
    // daily(mymove); //function
    let urmove = Move::Run;
    // println!("{:#?}",urmove);
    // urmove.ppt();//method

    // let mysweet = Sweet::Chocolate(String::from("Chocolate Heaven"));
    // let ursweet = Sweet::Open(String::from("Peanut Chikie"),1);
    // println!("{:#?}",mysweet);
    // println!("{:#?}",ursweet);
    // let seven = Option::Some(7);
    // let result = increment(seven);
    // println!("{:#?}",result);
    // let nothing:Option<i32> = Option::None;

    daily(mymove);
    daily(urmove);
}
