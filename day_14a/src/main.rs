use std::env::args;
fn main() {
    let mut a = args();
    a.next();
    let needed : usize = a.next().unwrap().parse().unwrap();
    let mut scoreboard : Vec<u32> = vec![3,7];
    let mut e1 = 0;
    let mut e2 = 1;
    while scoreboard.len() < needed+10 {
        let new_recipe = scoreboard[e1] + scoreboard[e2];
        if new_recipe >= 10 {
            scoreboard.push(1);
        }
        scoreboard.push(new_recipe % 10);
        e1 = (e1 + 1 + scoreboard[e1] as usize) % scoreboard.len();
        e2 = (e2 + 1 + scoreboard[e2] as usize) % scoreboard.len();
    }
    for n in scoreboard[needed..needed+10].iter() {
        print!("{}",n);
    }
    println!("");
}