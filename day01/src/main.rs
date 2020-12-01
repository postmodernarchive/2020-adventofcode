// basic idea is:
// 1. read file -> each line into an array/vector as u32
// 2.1. start from zero, add a[0] + a[1], a[0] + a[2]
// 2.2. next step start from one, add a[1] + a[2], a[1] + a[3]
//          and continue that way, each step you increase place in the array for first summand,
//          there is one less calclation the program has to do (obviously)
// 3. check, if the results equal 2020,
//  3.1 yes -> save both summands
//  3.2 no -> continue

fn main() {
    println!("Hello, world!");
}
