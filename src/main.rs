use std::io;
use rand::{random_range, rng};

fn main() {
    let guess = my_num();

    match_num(guess);
}

fn user_input() -> Result<i32, String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .map_err(|_|"Failed to read output")?;

    let num = input.trim().parse::<i32>().map_err(|_|"Enter a vaild number")?;

   Ok(num)
}

fn my_num() -> i32 {
    let guess: i32 = rand::random_range(1..10);
    guess
}

fn match_num(guess: i32) {
    let mut y=0;
    loop {
        let x = user_input();
        match x {
            Ok(x) => {
                if guess == x {
                    println!("You guess right");
                    y+=1;
                    break;
                } else {
                    println!("You guess wrong");
                    y+= 1;
                }
                
            }
            Err(_) => {
                println!("invaild number Try Again")
            }
        } 
    }

    println!("You guess in {} tries",y)
}
