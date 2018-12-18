use std::io::{BufRead, stdin};

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Clay,
    Pool,
    Flowing,
}
use crate::Cell::*;
impl Cell {
    fn is_solid(&self) -> bool {
        match *self {
            Empty | Flowing => false,
            Clay | Pool => true   
        }
    }

    fn is_water(&self) -> bool {
        match *self {
            Empty | Clay => false,
            Pool | Flowing => true
        }
    }

    fn to_ch(&self) -> char {
        match *self {
            Empty => '.',
            Clay => '#',
            Pool => '~',
            Flowing => '|'
        }
    }
}

fn mark_water(field: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    //println!("start {} {}", x, y);
    if y == (field.len()-1) {
        field[y][x] = Flowing;
        return;
    }

    let mut i = x;
    let left_bound = loop {
        if field[y][i].is_solid() {
            break (true, i+1);
        }
        if Empty == field[y+1][i] {
            //println!("  {}, {} is empty, recurse", i, y+1);
            mark_water(field, i, y+1);
        }
        //x=0 will always be an empty column, so this will always eventually break
        if !field[y+1][i].is_solid() {
            break (false, i);
        }
        i -= 1;
    };
    //println!("  left to {:?}", left_bound);
    let mut i = x;
    let right_bound = loop {
        if field[y][i].is_solid() {
            break (true, i-1);
        }
        if Empty == field[y+1][i] {
            mark_water(field, i, y+1);
        }
        //x=last will always be an empty column, so this will always eventuallybreak
        if !field[y+1][i].is_solid() {
            break (false, i);
        }
        i += 1;
    };
    //println!("  right to {:?}", right_bound);

    let fill = if left_bound.0 && right_bound.0 { Pool } else { Flowing };
    for i in left_bound.1..right_bound.1+1 {
        field[y][i] = fill;
    }
}

fn display(field: &Vec<Vec<Cell>>) {
    for row in field.iter() {
        for c in row.iter() {
            print!("{}", c.to_ch());
        }
        println!("");
    }
}

fn main() {
    let stdin = stdin();
    let veins : Vec<(usize, usize, usize, usize)> = stdin.lock().lines().map(|line| {
        let l = line.unwrap();
        let nums : Vec<usize>= l.split(|ch: char| !ch.is_numeric()).filter_map(|p| p.parse::<usize>().ok()).collect();
        if let Some('x') = l.chars().next() {
            (nums[0], nums[0], nums[1], nums[2])
        } else {
            (nums[1], nums[2], nums[0], nums[0])
        }
    }).collect();
    let min_x = veins.iter().map(|t| t.0).min().unwrap();
    let max_x = veins.iter().map(|t| t.1).max().unwrap();
    let min_y = veins.iter().map(|t| t.2).min().unwrap();
    let max_y = veins.iter().map(|t| t.3).max().unwrap();

    let mut field : Vec<Vec<Cell>> = (min_y..max_y+1).map(|_| {
        vec![Empty; max_x-min_x+1+2]
    }).collect();

    for (x1, x2, y1, y2) in veins.iter() {
        for y in *y1..y2+1 {
            for x in *x1..x2+1 {
                field[y - min_y][x - min_x + 1] = Clay;
            }
        }
    }

    display(&field);
    println!("");
    mark_water(&mut field, 500 - min_x + 1, 0);
    let count1 : usize = field.iter().map(|row| {
        row.iter().filter(|c| c.is_water()).count()
    }).sum();
    let count2 : usize = field.iter().map(|row| {
        row.iter().filter(|&&c| c == Pool).count()
    }).sum();
    display(&field);
    println!("{} {}", count1, count2);
}