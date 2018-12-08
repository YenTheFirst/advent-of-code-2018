use std::io::{stdin,BufRead};
use std::collections::HashSet;
fn main() {
    let stdin = stdin();
    let mut deps : Vec<(char,char)> = stdin.lock().lines().map(|l| {
        let line = l.unwrap();
        let mut words = line.split_whitespace();
        (
            words.nth(1).unwrap().chars().next().unwrap(),
            words.nth(5).unwrap().chars().next().unwrap()
        )
    }).collect();
    
    let (l,r): (HashSet<char>, HashSet<char>) = deps.iter().cloned().unzip();
    let mut uniq : HashSet<char> = l.union(&r).cloned().collect();
    
    while !uniq.is_empty() {
        let (_, blocked): (HashSet<char>, HashSet<char>) = deps.iter().cloned().unzip();
        let take = uniq.difference(&blocked).min().unwrap().clone();
        print!("{}", take);
        uniq.remove(&take);
        deps.retain(|(bl,_)| *bl != take);
    }
    println!("");
}