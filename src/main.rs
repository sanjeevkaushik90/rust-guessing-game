use std::io;

fn main() {
    let guess = my_num();

    match_num(guess);
}

fn user_input() -> Result<i32, String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Enter a vaild number");

    let num = input.trim().parse::<i32>();

    match num {
        Ok(num) => Ok(num),
        Err(_) => Err("Please enter a valid number".to_string()),
    }
}

fn my_num() -> i32 {
    let guess: i32 = 25;
    guess
}

fn match_num(guess: i32) {
    loop {
        let x = user_input();
        match x {
            Ok(x) => {
                if guess == x {
                    println!("You guess right");
                    break;
                } else {
                    println!("You guess wrong");
                }
            }
            Err(_) => {
                println!("invaild number Try Again")
            }
        }
    }
}
