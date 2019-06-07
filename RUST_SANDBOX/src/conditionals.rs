// Conditionals - Used to check the condition of something and act 
pub fn run(){
    let age: u8  = 18 ;
    let check_id: bool = false;
    let knows_porson_of_age = true;
    //If/Else
    if age >= 21 && check_id || knows_porson_of_age {
        println!("Lee: what would you like to drink ?");
    } else if age < 21 && check_id {
        println!("Lee: Sorry, you have to leave");
    } else {
        println!("Lee: I'll need to see your ID");
    }

    // Shorthand If 
    let is_of_age = if age >= 21 { true } else  { false };
    println!("Is of Age:  {}", is_of_age);
}