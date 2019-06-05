pub fn run() {
    println!("Hello from the print.rs file");

    //basic Formatting
    println!("{} love {}", "I", "You");
    // Postitional Arguments
    println!("{0} love {1} too {2} ", "I" , "you" , "dear");
    //Named Arguments 
    println!("{name} like to play {activaty}", name = "lee", activaty = "Game");
    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10 , 10 , 10);
    //basic math
    println!("10 + 10 = {}", 10 + 10 );
}