//variable hold primitive data or reference to data 
//variables are immutable by default 
//rust is a block-scoped language 
pub fn run() {
    let name = "Lee Kheang";
    let mut age = 23;
    println!("My name is {} and i am {}", name , age);
    age = 22;
    println!("My name is {} and i am {}", name , age);

    // Define constant  we need use datatype  
    const ID: i32 = 001;
    println!("ID: {}", ID);
    
    // Assign multiple vars 
    let ( my_name , my_age ) = ("Ni" , 22);
    println!("{} is {}" ,  my_name , my_age)
}