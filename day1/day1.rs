use std::fs; 

fn main() {

    let mut total_fuel = 0f64; 

    let f = fs::read_to_string("./masses.txt").unwrap(); //.expect("Unable to open file");
    //let masses = BufReader::new(f);

    for mass in f.lines() {
        let mod_mass = mass.parse::<f64>().unwrap();
        let mut addn_fuel = 0f64;

        let mut mod_fuel = (mod_mass / 3f64).floor() - 2f64; //Fuel calculation for the module mass only

        while mod_fuel >= 0f64 { // Calculate additional fuel for fuel
            addn_fuel += mod_fuel;
            mod_fuel = (mod_fuel / 3f64).floor() - 2f64;
        }

        total_fuel += addn_fuel; //Total fuel for the ship
    
    }

    print!("Total ship fuel is {} \n", total_fuel); 

}
