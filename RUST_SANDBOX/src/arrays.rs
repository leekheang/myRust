//Array Fixed list where elements are the same data types 
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    // Re-assign value
    numbers[2] = 20;

    // Get single val
    println!("show change {:?}", numbers);

    //Get array length
    println!("Array Length: {}", numbers.len());
    
    //Arrays are stack allocated std standardlib if you cute std on ur code you need declear it on top  like c ++
    println!("Array occupies {} bytes" , mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice:  {:?}", slice);

}