use std::env::args;
fn main() {
    let mut a = args();
    a.next();
    let mut target : u32 = a.next().unwrap().parse().unwrap();
    let mut digits = Vec::new();
    while target > 0 {
        digits.push(target % 10);
        target /= 10;
    }
    digits.reverse();

    let mut scoreboard : Vec<u32> = vec![3,7];
    let mut e1 = 0;
    let mut e2 = 1;
    while !scoreboard[scoreboard.len().saturating_sub(digits.len()).saturating_sub(2)..scoreboard.len()].windows(digits.len()).any(|w| w==digits.as_slice()) {
        let new_recipe = scoreboard[e1] + scoreboard[e2];
        if new_recipe >= 10 {
            scoreboard.push(1);
        }
        scoreboard.push(new_recipe % 10);
        e1 = (e1 + 1 + scoreboard[e1] as usize) % scoreboard.len();
        e2 = (e2 + 1 + scoreboard[e2] as usize) % scoreboard.len();
    }
    let i = scoreboard.windows(digits.len()).position(|w| w==digits.as_slice()).unwrap();
    println!("{}",i);
}