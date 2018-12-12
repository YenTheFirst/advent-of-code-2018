use std::io::BufRead;
use std::collections::{HashMap};


const GENERATIONS : u32 = 20;

//baking an assumption that input won't contain the rule '..... => #'
//which would make this problem undefined.
const PADDING : usize = GENERATIONS as usize + 2;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut field : Vec<char> = lines.next().unwrap().unwrap()
        .split_whitespace().nth(2).unwrap()
        .chars().collect();
    lines.next();
    let mut rules : HashMap<[char; 5],char> = HashMap::new();
    for l in lines {
        let ll = l.unwrap();
        let mut words = ll.split_whitespace();
        let mut pattern : [char; 5] = Default::default();
        pattern.copy_from_slice(&words.next().unwrap().chars().collect::<Vec<char>>()[0..5]);
        words.next();
        let result = words.next().unwrap().chars().next().unwrap();
        rules.insert(pattern, result);
    }

    println!("{}", field.len());
    for _ in (0..GENERATIONS) {
        field.resize(field.len()+8, '.');
        field.rotate_right(4);
        field = field.windows(5).map(|w| rules[w]).collect();
        println!("{}", field.len());
    }
    let r : i32 = field.iter().enumerate().filter(|&(_,&c)| c == '#').map(|(i,_)| i as i32 - (GENERATIONS*2) as i32).sum();
    println!("{}",r);
}

