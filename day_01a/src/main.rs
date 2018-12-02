use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let nums = lines.map(|l| l.unwrap().parse::<i32>().unwrap());
    println!("{}", nums.sum::<i32>());

}
