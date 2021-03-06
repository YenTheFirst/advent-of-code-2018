extern crate regex;
use regex::Regex;
use std::io::BufRead;

const WIDTH: usize = 1000;
fn main() {

    let stdin = std::io::stdin();
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let claims : Vec<(u32,u32,u32,u32,u32)> = stdin.lock().lines().map(|l| {
        let line = l.unwrap();
        let caps = re.captures(&line).unwrap();
        let id : u32 = caps[1].parse().unwrap();
        let x : u32 = caps[2].parse().unwrap();
        let y : u32 = caps[3].parse().unwrap();
        let w : u32 = caps[4].parse().unwrap();
        let h : u32 = caps[5].parse().unwrap();
        (x,y,x+w,y+h,id)
    }).collect();
    let mut sheet: [u32; WIDTH*WIDTH] = [0; WIDTH*WIDTH];

    for claim in &claims  {
        for y in claim.1..claim.3 {
            for x in claim.0..claim.2 {
                sheet[(y as usize)*WIDTH + (x as usize)] += 1;
            }
        }
    }

    let answer = claims.iter().find(|cl| {
        (cl.1..cl.3).all(|y| {
            (cl.0..cl.2).all(|x| {
                sheet[(y as usize)*WIDTH + (x as usize)] == 1
            })
        })
    }).unwrap().4;

    println!("{}", answer);
}
