
extern crate regex;
use regex::Regex;
use std::io::BufRead;
use std::cmp::max;

fn main() {
    let stdin = std::io::stdin();
    let re = Regex::new(r"position=<\s*(?P<px>-?\d+),\s*(?P<py>-?\d+)> velocity=<\s*(?P<vx>-?\d+),\s*(?P<vy>-?\d+)").unwrap();
    let (positions, velocities) : (Vec<(i32, i32)>, Vec<(i32, i32)>) = stdin.lock().lines().map(|l| {
        let line = l.unwrap();
        let caps = re.captures(&line).expect("couldn't match");
        (
            (
                caps["px"].parse().unwrap(),
                caps["py"].parse().unwrap(),
            ),
            (
                caps["vx"].parse().unwrap(),
                caps["vy"].parse().unwrap(),
            )
        )
    }).unzip();
    
    let range = 2 * max(
        positions.iter().map(|(x,_)| x).max().unwrap() - positions.iter().map(|(x,_)| x).min().unwrap(),
        positions.iter().map(|(_,y)| y).max().unwrap() - positions.iter().map(|(_,y)| y).min().unwrap()
    );

    let sky = (0..range).map(|t| {
        positions.iter().zip(velocities.iter()).map(|((px,py), (vx,vy))| (px+vx*t, py+vy*t)).collect::<Vec<(i32,i32)>>()
    });

    let (msg_t, msg) = sky.enumerate().min_by_key(|(_,poss)| {
        (poss.iter().map(|(x,_)| x).max().unwrap() - poss.iter().map(|(x,_)| x).min().unwrap()) +
        poss.iter().map(|(_,y)| y).max().unwrap() - poss.iter().map(|(_,y)| y).min().unwrap()
    }).unwrap();

    let min_x = msg.iter().map(|(x,_)| x).min().unwrap();
    let max_x = msg.iter().map(|(x,_)| x).max().unwrap();
    let min_y = msg.iter().map(|(_,y)| y).min().unwrap();
    let max_y = msg.iter().map(|(_,y)| y).max().unwrap();

    for y in *min_y..max_y+1 {
        for x in *min_x..max_x+1 {
            if msg.iter().find(|&&e| e == (x,y)).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n")
    }
    println!("{}", msg_t);
}