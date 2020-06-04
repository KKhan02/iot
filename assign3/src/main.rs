#[derive(Debug)]
struct Student{
    name:String,
    grade:String,
    age:u8,
    percentage:f32,
}

impl Student{
    fn instance(name:String,grade:String,age:u8,percentage:f32) -> Student {
        Student {
        name,
        grade,
        age,
        percentage,
     }
    }
    fn percent_print(&self){
        println!("{}",self.percentage);
    }
}

fn main() {
    let student1 = Student::instance(String::from("Khuzaim Khan IOT045022"),String::from("A+"),19,86.77);
    println!("{:#?}",student1);
    student1.percent_print();
}
