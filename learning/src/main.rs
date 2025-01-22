use searchingElement::searchingElements;

mod randomno; // Importiing the randomno module
mod shadowing; // shadow number
mod guess;
mod searchingElement;
mod looping;

// fn main() {
//    let secret_number = randomno::generate_number(1, 1000); //generate a rand no
//    randomno::play_guessing_game(secret_number);
// }


// fn main() {

//    shadowing::shadowing_example();

// }

// fn main() {

//    //guess::guessFunction();


//    // let t= true;
//    // let f: bool = false;
   
//    /* this is the portion of tuples */
//    //
//    let  tup: (i32, f64, u8) = (500, 6.4, 1);
//    let fiveHundered  = tup.0; //some compiler is showing me non snake case 
//    println!("number is {fiveHundered}");

// }

// fn main() {
//    let a = [1,2,3,4,5,6,7,8];// normal type 
//    let months = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];

//    let b:[i32;5] = [1,2,3,4,5]; //
//    let c = [3;5];    //this will be same as the [3,3,3,3,3]
//    let first = a[0];
//    let second = a[1];
//    searchingElement::searchingElements();

// }

fn main() {

   looping::looptest();
}

