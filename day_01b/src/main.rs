use std::io::BufRead;
use std::collections::HashSet;
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let nums : Vec<i32> = lines.map(|l| l.unwrap().parse::<i32>().unwrap()).collect();
    let mut sums = nums.iter().cycle().scan(0, |sum,elem| {
        *sum = *sum + elem;
        Some(*sum)
    });
    {
        let mut seen = HashSet::new();
        let repeat = sums.find(|s| !seen.insert((*s).clone())).unwrap();
        println!("{}", repeat);
    }
}
