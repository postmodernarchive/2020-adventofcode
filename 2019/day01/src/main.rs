use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: String = "input.txt".to_string();

    // fuel counter
    let mut module_fuel: u32 = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(number) = line {
                module_fuel += calc_fuel(number.parse::<u32>().unwrap());
            }
        }
    }

    println!("amount of fuel needed for modules is: {}", module_fuel);

    // calculate the remaining fule's fuel
    // (the mass of the fuel also requires more fuel)
    //let mut total_fuel = module_fuel;
    let mut total_fuel = module_fuel;
    let mut new_fuel: u32 = calc_fuel(total_fuel);

    while new_fuel >= 1 {
        total_fuel += new_fuel;
        new_fuel = calc_fuel(new_fuel);
        println!("{}", new_fuel);
    }

    println!("total amount of fuel needed for modules and fuel's fuel is: {}", total_fuel);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
// NOTE: straight up stolen from
// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html

pub fn calc_fuel(mass: u32) -> u32 {
    let result: i32 = (mass as i32 / 3) - 2;

    if result <= 0 {
        return 0
    } else {
        return result as u32;
    }
}

