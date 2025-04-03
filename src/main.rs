use rand::prelude::*;
use std::io;
use std::cmp::Ordering;

fn main(){
    println!("Pick any two numbers between 1 and 6, if their sum is greater than the numbers i pick, you win a million dollards");
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

    
    let mut roll:Vec<i32> = Vec::new();
    roll.push(*die.choose(&mut machine).expect("one"));
    roll.push(*die.choose(&mut machine).expect("two"));

    // let roll = vec![
    //     die.choose(&mut machine).expect("one"),
    //     die.choose(&mut machine).expect("two")
    // ];

    let rollsum = roll.iter().sum::<i32>();
    let choicesum = choices.iter().sum::<i32>();
    println!("The sum of your choices is {:?}", choicesum);

    match rollsum.cmp(&choicesum){
        Ordering::Less => println!("Too High! The sum of my die is {:?}, less than your {:?}", rollsum, choicesum),
        Ordering::Greater =>println!("Too Low! The sum of my die is {:?}, greater than your {:?}", rollsum, choicesum),
        Ordering::Equal =>println!("Great! {:?} is equal to {:?}", rollsum, choicesum),
    }
   
    

}