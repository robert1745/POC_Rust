fn main() {
    // let x : i32 = -5;
    // let y : u32 = 10;
    // let z : f32 = 3.14;
    // print!("x: {} y: {} z: {}\n", x, y, z);

    // Overflowing an i8 type
    // let mut x : i8 = 100; -> we add mut infront of x so that we can change it
    // By default , all variables are immutable in Rust

    // for i in 0..1000 {
    //     x = x + 100
    // }
    // println!("x: {}", x);

    // String in Rust

    let greeting : &str = "Hello, world!";
    // println!("{}", greeting);

    let greet = String::from("Hello, world!");
    // println!("{}", greet);

    // Accessing characters in a string directly is not allowed in Rust
    // because strings are UTF-8 encoded and can contain multi-byte characters.
    // println!("{}", greet[0]); // This will cause an error


    // Instead, we can use the chars() method to iterate over characters
    // for c in greet.chars() {
    //     print!("{} ", c);
    // }

    // but if i wnat to acceess a specific character let say 5
    // 1st method using match
    match greet.chars().nth(5) {
        Some(c) => println!("\nCharacter at index 5: {}", c),
        None => println!("\nNo character at index 5"),
        
    }

    // 2nd method 
    let index : usize = 5;
    let val = greet.chars().nth(index);
    println!("val: {}", val.unwrap_or(' '));


}

