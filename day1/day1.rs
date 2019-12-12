extern crate math;

use std::fs::File; 
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, Write};
use math::round;

fn main() {

    let total_fuel = 0; 

    if let Ok(masses) = read_line("./masses.txt") {
    
        for mass in masses {
            let mass_int = mass.parse::<u32>();
            let total_fuel += (round::floor((mass_int / 3), 0) - 2)

        }
    }

   print!("{}", total_fuel); 
}
