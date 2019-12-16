use std::fs::File; 
use std::io::{self,BufReader};
use std::io::prelude::*;

fn main() {

    let total_fuel = 0; 

    let masses = File::open("masses.txt")?;
    let masses = BufReader::new(masses);

    for mass in masses.lines() {
        let mass_int = mass.parse::<u32>();
        let total_fuel = total_fuel + (mass_int / 3).floor() - 2;
    }

   print!("{}", total_fuel); 
}
