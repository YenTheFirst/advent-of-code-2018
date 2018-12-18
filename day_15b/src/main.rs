use std::io::BufRead;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone)]
enum Team {
    Goblin,
    Elf
}
#[derive(Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Unit {
        team: Team,
        health: i32
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash, Debug)]
struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    // a bit hacky, assume that x, y will never be max, as there's always a wall there.
    fn neighbors(&self) -> Vec<Pos> {
        vec![
            Pos { x: self.x, y: self.y-1 },
            Pos { x: self.x-1, y: self.y },
            Pos { x: self.x+1, y: self.y },
            Pos { x: self.x, y: self.y+1 },
        ]
    }
}

fn main() {
    let stdin = std::io::stdin();
    let grid: Vec<Vec<char>> = stdin.lock().lines().map(|l| {
        l.unwrap().chars().collect()
    }).collect();
    let n = (1..100).map(|n| (n, simulate_combat(&grid, n))).find(|(_,(_,ed))| *ed == 0);
    println!("{:?}", n)
}

fn simulate_combat(grid: &Vec<Vec<char>>, elf_power: i32) -> (i32, i32) {
    let mut num_goblins = 0;
    let mut num_elves = 0;
    let mut field: Vec<Vec<Cell>> = grid.iter().map(|l| {
        l.iter().map(|ch| {
            match ch {
                '.' => Cell::Empty,
                '#' => Cell::Wall,
                'G' => {num_goblins += 1; Cell::Unit { team: Team::Goblin, health: 200 }},
                'E' => {num_elves += 1; Cell::Unit { team: Team::Elf, health: 200 }},
                _ => unreachable!()
            }
        }).collect()
    }).collect();

    let mut num_rounds = 0;
    let mut elf_deaths = 0;
    'outer: while num_elves > 0 && num_goblins > 0 {
        let positions : Vec<Pos> = field.iter().enumerate().flat_map(|(y,row)| {
            row.iter().enumerate().filter_map(move |(x,cell)| {
                match cell {
                    Cell::Unit{ team: _, health: _ } => Some(Pos { x: x, y: y }),
                    _ => None
                }
            })
        }).collect();

        for pos in positions.iter() {
            if let Cell::Unit { team: t, health: h } = field[pos.y][pos.x].clone() {
                if h <= 0 { continue }
                //try to move to target
                let next_step = path_to_target(&field, pos, &t);
                let new_pos = if let Some(p2) = next_step {
                    println!("{:?} to {:?}", pos, p2);
                    field[p2.y][p2.x] = field[pos.y][pos.x].clone();
                    field[pos.y][pos.x] = Cell::Empty;
                    p2
                } else {
                    println!("{:?} has no path to targets", pos);
                    pos.clone()
                };
                //if next to target, attack
                let n = new_pos.neighbors();
                let target = n.iter().filter_map(|np| {
                    match &field[np.y][np.x] {
                        Cell::Unit { team: t2, health: h2 } if *t2 != t && *h2 >= 0 => Some((np, h2)),
                        _ => None
                    }
                }).min_by_key(|(_,&h)| h);
                println!("after moving, {:?} attacks {:?}", new_pos, target);
                if let Some((p2, _)) = target {
                    if let Cell::Unit{team: u_t, health: u_h} = &mut field[p2.y][p2.x] {
                        *u_h -= match t {
                            Team::Goblin => 3,
                            Team::Elf => elf_power
                        };
                        if *u_h <= 0 {
                            if *u_t == Team::Elf {
                                num_elves -= 1;
                                elf_deaths += 1;
                            } else {
                                num_goblins -= 1;
                            }
                            field[p2.y][p2.x] = Cell::Empty;
                            if num_elves <= 0 || num_goblins <= 0 || elf_deaths > 0 {
                                break 'outer;
                            }
                        }
                    }
                }
            }
            //print_field(&field);
        }
        //off-by-one on round count, might be worth fixing.
        num_rounds += 1;
        println!("after {} rounds", num_rounds);
        print_field(&field);
    }
    print_field(&field);

    let s : i32 = field.iter().flat_map(|row| {
        row.iter().filter_map(|cell| {
            match cell {
                Cell::Unit{ health: h, team: _ } if *h > 0 => Some(h),
                _ => None
            }
        })
    }).sum();
    println!("{}", s*num_rounds);
    (s*num_rounds, elf_deaths)
}

fn path_to_target(field : &Vec<Vec<Cell>>, start: &Pos, my_team: &Team) -> Option<Pos> {
    let mut frontiers : Vec<HashSet<Pos>> = Vec::new();
    frontiers.push([start.clone()].iter().cloned().collect());
    let mut destinations = loop {
        let cur_frontier = frontiers.last().unwrap();
        let all_reachable : HashSet<Pos> = cur_frontier.iter().map(|p| p.neighbors()).flatten().collect();
        let reachable : HashSet<Pos> = if frontiers.len() >= 2 {
            all_reachable.difference(&frontiers[frontiers.len()-2]).cloned().collect()
        } else {
            all_reachable
        };
        println!("frontier at {} reachable = {:?}", frontiers.len(), reachable);
        let enemies : HashSet<Pos> = reachable.iter().filter(|p| {
            if let Cell::Unit { team: t, health: h } = &field[p.y][p.x] {
                *h > 0 && t != my_team
            } else {
                false
            }
        }).cloned().collect();
        if !enemies.is_empty() {
            break enemies;
        }
        let next_frontier : HashSet<Pos> = reachable.iter().filter(|p| field[p.y][p.x] == Cell::Empty).cloned().collect();
        println!("frontier at {} next_frontier = {:?}", frontiers.len(), next_frontier);
        if next_frontier.is_empty() {
            return None;
        } else {
            frontiers.push(next_frontier);
        }
    };

    if frontiers.len() < 2 {
        return None;
    }
    println!("enemies: {:?}", destinations);
    for f in frontiers[1..frontiers.len()].iter().rev() {
        let neighbors = destinations.iter().map(|p| p.neighbors()).flatten().collect();
        destinations = f.intersection(&neighbors).cloned().collect();
        println!("backtrack {:?}", destinations);
    }
    destinations.iter().min().map(|p| p.clone())
}

fn print_field(field: &Vec<Vec<Cell>>) {
    for row in field.iter() {
        for c in row.iter() {
            print!("{}", match c {
                Cell::Empty => '.',
                Cell::Wall => '#',
                Cell::Unit { team: Team::Goblin, health: _ } => 'G',
                Cell::Unit { team: Team::Elf, health: _ } => 'E',
            })
        }
        for u in row.iter() {
            match u {
                Cell::Unit { team: Team::Goblin, health: h } => print!(" G({})",h),
                Cell::Unit { team: Team::Elf, health: h } => print!(" E({})",h),
                _ => ()
            }
        }
        println!("");
    }
    println!("");
}