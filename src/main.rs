use rand::prelude::*;
use std::io;
use std::cmp::Ordering;

fn main(){
    println!("Pick any two numbers, if their sum is greater than the numbers i pick, you win a million dollards");
    let mut x = String::new();
    let mut y = String::new();

    println!("The first number is...");
    io::stdin()
        .read_line(&mut x)
        .expect("Horrors");

    println!("The second is...");
    io::stdin()
        .read_line(&mut y)
        .expect("A Minooooooooor");
    let choices = vec![
        x.trim().parse::<i32>().expect("Hey Now"),
        y.trim().parse::<i32>().expect("Say now")
    ];

    let mut machine = rand::rng();
    let mut die:Vec<i32> = (1..=6).collect();

    let roll = vec![
        die.choose(&mut machine).expect("one"),
        die.choose(&mut machine).expect("two")
    ];

    println!("The roll is {:?}, your choices were {:?}",roll, choices );

    match roll.iter().sum().cmp

    

}