// Struct - Used to create data types


struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

//Tuple Struct 
struct Colors(u8, u8, u8);

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
}