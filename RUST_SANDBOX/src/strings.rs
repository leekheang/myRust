//primitive str = immutale fixed-length string somewhere in memory 
//String = Growable , heap-allocated data structure - Use when you need to modify or own string data 

pub fn run() {
    let mut hello = String::from("Hello ");
    //push char
    hello.push('W');
    //push string
    hello.push_str("orld!");
    //Capacity  of bytes
    println!("Capacity: {}", hello.capacity());
    //check if empty
    println!("Is Empty : {}", hello.is_empty());
    //Contains 
    println!("Contains 'World' {}" , hello.contains("World"));
    //Replace
    println!("Replace: {}", hello.replace("World", "Mother fuck"));
    //Loop Through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }
   
    println!("{}", hello);

    //Create string with capacity 
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    // Assetion testing
    assert_eq!(2, s.len()); //eq == 
    println!("{}", s);
}
