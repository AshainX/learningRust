use std::io;

pub fn searchingElements(){
    let a = [1,2,3,4,5];

    println!("input number");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was NaN");

    let element = a[index];

    println!("the val at index {index} is: {element}");

}