use std::fs::File; 
use std::io::{BufRead,BufReader};

fn main() {

    let mut total_fuel = 0f64; 

    let f = File::open("./masses.txt").expect("Unable to open file");
    let masses = BufReader::new(f);

    for mass in masses.lines() {
        let mass_int = mass.unwrap().parse::<f64>();
        total_fuel += (mass_int.unwrap() / 3f64).floor() - 2f64;
    }

   print!("{} \n", total_fuel); 
}
