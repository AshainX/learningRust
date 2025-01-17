pub fn shadowing_example() {
    let x = 5;
    println!("First value of x: {}", x);

    let mut y = 4;
    y+=2;
    println!("After mutating with y as 4 then the mutated y is {}",y );

    let x = x + 2; 
    println!("After shadowing, value of x: {}", x);

    let x = "Now I'm a string!"; 
    println!("After another shadowing, value of x: {}", x);
}