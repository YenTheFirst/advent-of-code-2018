use std::io::BufRead;
use std::collections::HashMap;
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let (twos, threes) : (Vec<bool>, Vec<bool>) = lines.map(|l| {
        let mut c = HashMap::new();
        for ch in l.unwrap().chars() {
            let e = c.entry(ch).or_insert(0);
            *e += 1;
        };
        (
            c.values().any(|&v| v==2),
            c.values().any(|&v| v==3)
        )
    }).unzip();
    let count2 = twos.iter().filter(|&&x|x).count();
    let count3 = threes.iter().filter(|&&x|x).count();
    println!("{}", count2*count3);
}
