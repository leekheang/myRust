// Primitive Types--
// Integers: u8. i8, u16, i16, u32, i32, u64, i64, u128, i128 (number if bits they take in memory)
// Float: f32, f64
// Boolean (bool)
// Characters (char)
// Tuples
// Arrays

// Rust is a statically typed language , which means that it must know the types of all 
// variable at compile time , however , the compiler can usually infer what type we want to use
// base on the value and how to use it 

pub fn run() {
    // default is "i32"
    let _x = 1 ; 
    //default is "f64"
    let _z = 2.5;
    //add explicit type 
    let _y: i64 = 43545435435435;
    //Find max size 
    println!("result is {}", _z + 1.0 );
    println!("Max i32: {}",std::i32::MAX);
    println!("Max i64: {}",std::i64::MAX);

}