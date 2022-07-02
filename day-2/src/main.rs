use rand::distributions::{Distribution, Uniform};
use std::io::{stdin, stdout, Write};
fn main() {
    print_options();

    let mut rng = rand::thread_rng();

    let x: Uniform<i32> = Uniform::from(1..4);
    let guess = x.sample(&mut rng);

    let mut num = 0;

    while num == 0 {
        print!("> ");
        let n = read_num();
        if n != 0 && n < 4 {
            num = n;
        } else {
            println!("input failed try again");
        }
    }

    win_conditions(num, guess)
}

fn read_num() -> i32 {
    let mut s = String::new();

    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("not a string");
    let s: i32 = match s.trim().parse::<i32>() {
        Ok(parsed_input) => parsed_input,
        Err(_) => {
            println!("Please enter a valid number");
            return 0;
        }
    };
    s
}

fn print_options() {
    println!("1) rock");
    println!("2) paper");
    println!("3) sissors");
}

fn num_to_str(number: i32) -> &'static str {
    match number {
        1 => "rock",
        2 => "paper",
        3 => "sissors",
        _ => "invalid",
    }
}

fn win_conditions(user: i32, computer: i32) {
    if user == computer {
        println!("draw");
    } else if user == 1 && computer == 2 || user == 2 && computer == 3 || user == 3 && computer == 1
    {
        println!("you win");
        println!("{} beats {}!", num_to_str(user), num_to_str(computer))
    } else {
        println!("you lose");
        println!("{} beats {}!", num_to_str(computer), num_to_str(user))
    }
}
