use rand::Rng;
use std::io;

fn main() {
    println!("This program is for playing rock, paper, scissors against me!");
    println!("You have to choose one of either 'rock', 'paper' or 'scissors'. 
I will choose before you write anything so you don't think I'm cheating, then I'll tell you who won ;)");

    let possible_values = ["rock", "paper", "scissors"];

    let user_value: String = read_value(possible_values);
    println!("You chose: {user_value}");

    let machine_value: String = choose_value(possible_values);
    println!("I chose: {}", machine_value);

    println!("{}", verdict_msg(&user_value, &machine_value));
}

fn read_value(possible_values: [&str; 3]) -> String {
    loop {
        let mut user_value = String::new();

        io::stdin()
            .read_line(&mut user_value)
            .expect("Failed to read line");

        let user_value: String = match user_value.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        for possible_value in possible_values.iter() {
            if user_value == possible_value.to_string() {
                return user_value;
            }
        }
        println!("The value must be either 'rock', 'paper' or 'scissors'. Please confirm you have written your value correctly.");
    }
}

fn choose_value(possible_values: [&str; 3]) -> String {
    let index = rand::thread_rng().gen_range(0..=2);
    possible_values[index].to_string()
}

fn verdict_msg(user_value: &str, machine_value: &str) -> String {
    if user_value == machine_value {
        return "It was a tie.".to_string()
    } else {
        if user_value.to_string() == "rock" {
            if machine_value.to_string() == "paper" {
                return "Sorry. I won ;)".to_string()
            } else {
                return "Congrats! You won :)".to_string()
            }
        } else if user_value.to_string() == "paper" {
            if machine_value.to_string() == "scissors" {
                return "Sorry. I won ;)".to_string()
            } else {
                return "Congrats! You won :)".to_string()
            }
        } else { // if user_value.to_string() == "scissors"
            if machine_value.to_string() == "rock" {
                return "Sorry. I won ;)".to_string()
            } else {
                return "Congrats! You won :)".to_string()
            }
        }
    }
}