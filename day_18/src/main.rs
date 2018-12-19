use std::io::{stdin,BufRead};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Cell {
    Empty,
    Tree,
    Lumberyard,
}
use crate::Cell::*;

fn main() {
    let stdin = stdin();
    let mut grid: Vec<Vec<Cell>> = stdin.lock().lines().map(|l| {
        l.unwrap().chars().map(|ch| {
            match ch {
                '.' => Empty,
                '|' => Tree,
                '#' => Lumberyard,
                _ => unreachable!()
            }
        }).collect()
    }).collect();
    let mut seen : HashMap<Vec<Vec<Cell>>, u64> = HashMap::new();
    //let mut repeat : Option<(u64, u64)> = None;


    //for _ in 0..10 {
    for i in 0..1000000000 {
    //for i in 0..100 {
        grid = (0..grid.len() as i32).map(|y| {
            (0..grid[y as usize].len() as i32).map(|x| {
                let count_tree : usize = [(x-1,y-1), (x,y-1), (x+1,y-1), (x-1,y), (x+1,y), (x-1,y+1), (x,y+1), (x+1,y+1)].iter().filter(|(nx, ny)| grid.get(*ny as usize).and_then(|r| r.get(*nx as usize)) == Some(&Tree)).count();
                let count_yard : usize = [(x-1,y-1), (x,y-1), (x+1,y-1), (x-1,y), (x+1,y), (x-1,y+1), (x,y+1), (x+1,y+1)].iter().filter(|(nx, ny)| grid.get(*ny as usize).and_then(|r| r.get(*nx as usize)) == Some(&Lumberyard)).count();
                match (grid[y as usize][x as usize], count_tree, count_yard) {
                    (Empty, tree, _) if tree >= 3 => Tree,
                    (Empty, _, _) => Empty,
                    (Tree, _, yard) if yard >= 3 => Lumberyard,
                    (Tree, _, _) => Tree,
                    (Lumberyard, tree, yard) if tree >= 1 && yard >= 1 => Lumberyard,
                    (Lumberyard, _, _) => Empty
                }
            }).collect()
        }).collect();
        if seen.contains_key(&grid) {
            let j = seen[&grid];
            
            let index_at_end = ((1000000000 - 1 - j) % (i - j)) + j;
            //let index_at_end = ((100 - j) % (i - j + 1)) + j;
            println!("repeat at {} {}. iae = {}", j, i, index_at_end);
            //let at_end = seen.iter().find(|(seen,ix)| **ix == index_at_end).unwrap().0;
            for (at_end, ix) in seen.iter() {
                let count_tree : usize = at_end.iter().map(|row| row.iter().filter(|&&c| c == Tree).count()).sum();
                let count_yard : usize = at_end.iter().map(|row| row.iter().filter(|&&c| c == Lumberyard).count()).sum();
                println!("{}: {}", ix, count_tree*count_yard);    
            }
            return;
        }
        seen.insert(grid.clone(), i);
        let count_tree : usize = grid.iter().map(|row| row.iter().filter(|&&c| c == Tree).count()).sum();
        let count_yard : usize = grid.iter().map(|row| row.iter().filter(|&&c| c == Lumberyard).count()).sum();
        println!("{}: {}", i, count_tree*count_yard);
    }
    println!("done");
    let count_tree : usize = grid.iter().map(|row| row.iter().filter(|&&c| c == Tree).count()).sum();
    let count_yard : usize = grid.iter().map(|row| row.iter().filter(|&&c| c == Lumberyard).count()).sum();
    println!("{}", count_tree * count_yard);
}
