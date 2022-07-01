use rand::distributions::{Distribution, Uniform};
use std::io::{stdin, stdout, Write};

fn main() {
    println!("1) rock");
    println!("2) paper");
    println!("3) sissors");

    let mut rng = rand::thread_rng();

    let mut s = String::new();

    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("not a string");
    let s: i32 = s.trim().parse().expect("not a number");
    let x: Uniform<i32> = Uniform::from(1..4);
    let guess = x.sample(&mut rng);
    println!("generated {} input {}", test(guess), test(s));

    if guess == s {
        println!("draw");
    } else if guess == 1 && s == 2 || guess == 2 && s == 3 || guess == 3 && s == 1 {
        println!("you win");
    } else {
        println!("you lose");
    }
}

fn test(number: i32) -> &'static str {
    match number {
        1 => "rock",
        2 => "paper",
        3 => "sissors",
        _ => "lol",
    }
}
