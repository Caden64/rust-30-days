use rand::distributions::{Distribution, Uniform};
use std::io::{stdin, stdout, Write};

fn main() {
    print_options();

    let mut rng = rand::thread_rng();

    let s = read_num();

    let x: Uniform<i32> = Uniform::from(1..4);
    let guess = x.sample(&mut rng);
    //println!("generated {} input {}", num_to_str(guess), num_to_str(s));

    win_conditions(s, guess)
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

fn win_conditions(user: i32, computer: i32) {
    if user == computer {
        println!("draw");
    } else if user == 1 && computer == 2 || user == 2 && computer == 3 || user == 3 && computer == 1 {
        println!("you win");
        println!("{} beats {}!", num_to_str(user), num_to_str(computer))
    } else {
        println!("you lose");
        println!("{} beats {}!", num_to_str(computer), num_to_str(user))
    }
}
