// basic idea is:
// 1. read file -> each line into an array/vector as u32
// 2.1. start from zero, add a[0] + a[1], a[0] + a[2]
// 2.2. next step start from one, add a[1] + a[2], a[1] + a[3]
//          and continue that way, each step you increase place in the array for first summand,
//          there is one less calclation the program has to do (obviously)
// 3. check, if the results equal 2020,
//  3.1 yes -> save both summands
//  3.2 no -> continue

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut filename: String = "input.txt".to_string();

    if std::env::args().nth(1) != None {
        filename = std::env::args().nth(1).unwrap();
    }

    // vector for numbers
    let mut num_vec: Vec<u32> = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(number) = line {
                num_vec.push(number.parse::<u32>().unwrap());
            }
        }
    }

    let mut sum1: usize = 0;
    let mut sum2: usize = 1;
    let mut result: u32 = 0;
    let len: usize = num_vec.len();

    // vec[0..1] + vec[1..2]
    // vec[0..1] + vec[2..3]

    while result != 2020 && sum1 < len {
        for iter in &num_vec {
            result = num_vec[sum1..sum1+1][0] + iter;
            sum2 = *iter as usize;
        }

        sum1 += 1;
    }

    //println!("{} and {} equal {}", num_vec[sum1..sum1+1][0], num_vec[sum2..sum2+1][0], result);

    //println!("{:#?}", num_vec);
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
