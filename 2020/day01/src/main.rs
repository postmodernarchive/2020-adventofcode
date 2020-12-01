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

    // part1
    let (num1, num2) = part1(&num_vec);
    println!("{} + {} = {}", num1, num2, num1+num2);
    // now I need to multiply the two
    println!("{} * {} = {}", num1, num2, num1*num2);

    // part2
    let (num1, num2, num3) = part2(&num_vec);
    println!("{} + {} + {} = {}", num1, num2, num3, num1+num2+num3);
    // now I need to multiply the two
    println!("{} * {} * {} = {}", num1, num2, num3, num1*num2*num3);
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

// solution to part 1
fn part1(vector: &Vec<u32>) -> (u32, u32) {
    let mut num1: u32= 0;
    let mut num2: u32 = 1;
    // length of the vector
    let len: usize = vector.len();

    'out : for s1 in 0..len {
        for s2 in s1..len {
            let sum = vector[s1] + vector[s2];
            if sum == 2020 {
                num1 = vector[s1];
                num2 = vector[s2];
                break 'out;
            }
        }
    }

    (num1, num2)
}

// solution to part 2
fn part2(vector: &Vec<u32>) -> (u32, u32, u32) {
    let mut num1: u32= 0;
    let mut num2: u32 = 1;
    let mut num3: u32 = 2;
    // length of the vector
    let len: usize = vector.len();

    'out : for s1 in 0..len {
        for s2 in s1..len {
            for s3 in s2..len {
                let sum = vector[s1] + vector[s2] + vector[s3];
                if sum == 2020 {
                    num1 = vector[s1];
                    num2 = vector[s2];
                    num3 = vector[s3];
                    break 'out;
                }
            }
        }
    }

    (num1, num2, num3)
}
