//Vector Fixed list where elements are the same data types 
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);
    // Re-assign value
    numbers[2] = 20;

    //Add on to vector 
    numbers.push(5);
    // Pop off last value
    numbers.pop();

    // Get single val
    println!("show change {:?}", numbers);

    //Get array length
    println!("Vector  Length: {}", numbers.len());
    
    //Arrays are stack allocated std standardlib if you cute std on ur code you need declear it on top  like c ++
    println!("Vector occupies {} bytes" , mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice:  {:?}", slice);
    //Loop though vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }
    //Loop & mutate values
    for x in numbers.iter_mut() {
        *x *=2;
    }
    println!("Number Vec: {:?}", numbers);

}