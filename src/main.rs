use std::io;
use regex::Regex;

fn main() {
    loop {
        let weight = get_input("Input a weight (or 'q' to quit)");
        if is_quit(&weight) {
            println!("Exiting!");
            break;
        }

        let weight: f32 = parse_input(&weight);

        let reps = get_input("Input reps (or 'q' to quit)");
        if is_quit(&reps) {
            println!("Exiting!");
            break;
        }

        let reps: f32 = parse_input(&reps);

        println!("Your one rep max is {}", calc_max(weight, reps));
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn is_quit(input: &str) -> bool {
    let quit_regex = Regex::new(r"^q$").unwrap();
    quit_regex.is_match(input)
}

fn parse_input(input: &str) -> f32 {
    match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            std::process::exit(1);
        }
    }
}

fn calc_max(weight: f32, reps: f32) -> f32 {
    weight / ( 1.0278 - 0.0278 * reps )
}
