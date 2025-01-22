use std::io::{self, Read};



pub fn looptest(){
   // let mut count =0;
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed");

    let num: i32 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Enter valid number");
            return;
        }
    };
        // .trim()
        // .map(|x| x.parse().expect("Not a valid number"))
        // .collect();

    if num <5 {

        println!("Its less than 5");
    }
    // else {
    //     println!("Condition false");  //this should be in a block otherwise itwill throw an error
    // }

    else if num > 5 {
        println!(" Its greater than 5");
    }
    else{
        println!("Ok you are at 5")
    }

}