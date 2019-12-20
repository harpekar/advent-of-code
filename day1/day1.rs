use std::fs::File; 
use std::io::{BufRead,BufReader};

fn main() {

    let mut total_fuel = 0f64; 

    let f = File::open("./masses.txt").expect("Unable to open file");
    let masses = BufReader::new(f);

    for mass in masses.lines() {
        let mass_int = mass.unwrap().parse::<f64>();
        total_fuel += (mass_int.unwrap() / 3f64).floor() - 2f64; //Total fuel for the ship itself
    }

    print!("Total ship fuel is {} \n", total_fuel); 

    let mut addn_fuel = total_fuel;

    while addn_fuel >= 0f64 {
        addn_fuel = (addn_fuel / 3f64).floor() - 2f64;
        total_fuel += addn_fuel;
    }

    print!("Total ship fuel including fuel is {} \n", total_fuel); //Total  
}
