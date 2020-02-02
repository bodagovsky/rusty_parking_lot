mod model;
mod tests;
mod logic;
use std::{io};
use crate::model::{ParkingLot};
use std::env;
use std::fs;


fn main() {
    let mut parking_lot = ParkingLot{
        capacity:0,
        slots: Default::default()
    };
    let args:Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file = fs::read_to_string(&args[1]).expect(format!("Failed to open file {}", args[1]).as_ref());
        let inputs:Vec<&str> = file.split("\n").collect();
        for line in inputs {
            let mut input:Vec<&str> = line.split(" ").collect();
            let result = logic::process(&mut input, &mut parking_lot);
            println!("{}", result)
        }
    }

    println!("Welcome to the parking lot\nType `help` for list of commands");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read input");
        let mut split_input: Vec<&str> = input.trim().split(' ').collect();
        let result = logic::process(&mut split_input, &mut parking_lot);
        println!("{}", result)
    }

}
