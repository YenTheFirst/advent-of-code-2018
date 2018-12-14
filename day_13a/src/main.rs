use std::io::BufRead;

enum Track {
    None,
    Straight,
    LeftCurve,
    RightCurve,
    Intersection
}
#[derive(Clone)]
enum Turn {
    Left,
    Straight,
    Right
}
#[derive(Clone)]
struct Cart {
    pos : (i32, i32),
    dir: (i32, i32),
    next_turn: Turn
}
fn main() {
    let stdin = std::io::stdin();
    let grid: Vec<Vec<char>> = stdin.lock().lines().map(|l| {
        l.unwrap().chars().collect()
    }).collect();

    let tracks : Vec<Vec<Track>> = grid.iter().map(|row| {
        row.iter().map(|ch| {
            match ch {
                '-' | '|' | 'v' | '^' | '<' | '>' => Track::Straight,
                '/' => Track::LeftCurve,
                '\\' => Track::RightCurve,
                '+' => Track::Intersection,
                ' ' => Track::None,
                _ => panic!("unknown char {}", ch)
            }
        }).collect()
    }).collect();

    let mut carts : Vec<Cart> = grid.iter().enumerate().flat_map(|(y,row)| {
        row.iter().enumerate().filter_map(move |(x, ch)| {
            match ch {
                '^' => Some((0,-1)),
                'v' => Some((0,1)),
                '>' => Some((1,0)),
                '<' => Some((-1,0)),
                _ => None
            }.map(|dir| Cart{pos: (x as i32,y as i32), dir: dir, next_turn: Turn::Left})
        })
    }).collect();

    loop {
        carts.sort_by_key(|c| (c.pos.1, c.pos.0));
        for i in 0..carts.len() {
            //move
            let new_pos = {
                println!("cart {} starts at {:?}",i,carts[i].pos);
                carts[i].pos = (carts[i].pos.0 + carts[i].dir.0, carts[i].pos.1 + carts[i].dir.1);
                carts[i].pos
            };
            //cart.pos = (cart.pos.0 + cart.dir.0, cart.pos.1 + cart.dir.1);
            //crash
            if carts.iter().enumerate().any(|(j,c)| i != j && new_pos == c.pos) {
                println!("{:?}",new_pos);
                return;
            }
            println!("cart {} is at {:?}",i,new_pos);
            //turn
            let mut cart = &mut carts[i];
            cart.dir = match tracks[cart.pos.1 as usize][cart.pos.0 as usize] {
                Track::Straight => cart.dir,
                //  |   /-
                // -/   |
                Track::LeftCurve => {
                    match cart.dir {
                        (1,0) => (0,-1),
                        (0,1) => (-1,0),
                        (0,-1) => (1,0),
                        (-1,0) => (0,1),
                        _ => unreachable!()
                    }
                }
                // -\   |
                //  |   \-
                Track::RightCurve => {
                    match cart.dir {
                        (1,0) => (0,1),
                        (0,-1) => (-1,0),
                        (0,1) => (1,0),
                        (-1,0) => (0,-1),
                        _ => unreachable!()
                    }
                }
                Track::Intersection => {
                    match cart.next_turn {
                        Turn::Left => {cart.next_turn = Turn::Straight; (cart.dir.1, -cart.dir.0)},
                        Turn::Straight => {cart.next_turn = Turn::Right; cart.dir},
                        Turn::Right => {cart.next_turn = Turn::Left; (-cart.dir.1, cart.dir.0)},
                    }
                }
                Track::None => panic!("off the tracks")
            }
        }
    }
}
