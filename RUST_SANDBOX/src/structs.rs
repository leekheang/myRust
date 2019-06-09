// Struct - Used to create data types


struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

//Tuple Struct 
struct Colors(u8, u8, u8);

//Struct Person
struct Person {
    first_name: String,
    last_name: String
}
impl Person {
    //Construct person
    fn new(first: &str,last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }   
    }
    //Get full name 
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name,self.last_name)
    }
    //Set last name 
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }
    //Name to tuple
    fn to_tuple(self) -> (String , String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let  c  = Color {
        red: 255, 
        green: 0,
        blue: 0,
    };

    println!("Struct Color: {} {} {}", c.red, c.green, c.blue );

    let mut x = Colors(255, 1, 1);
    x.1 = 2;
    println!("Tuple Sruct Color {} {} {}", x.0 , x.1 , x.2 );

    let mut  p = Person::new("John", "Wick");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Full Name Of Person : {}", p.full_name());
    p.set_last_name("Lee");
    println!("Set Last but full : {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}