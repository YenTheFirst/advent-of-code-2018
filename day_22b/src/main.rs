const MODULO : u64 = 20183;
//big enough that it may be worth thinking about actually generating terrain on-demand, not in-advance
const BUFFER : usize = 1000;
use std::collections::{HashSet,BinaryHeap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Tool {
    None,
    Torch,
    Gear,
}
use crate::Tool::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct State {
    x: u64,
    y: u64,
    tool: Tool
}

impl State {
    fn neighbors(&self) -> Vec<(State, u64)> {
        vec![
            ( State { x : self.x.saturating_sub(1), y: self.y, tool: self.tool }, 1 ),
            ( State { x : self.x, y: self.y.saturating_sub(1), tool: self.tool }, 1 ),
            ( State { x: self.x+1, y: self.y, tool: self.tool }, 1 ),
            ( State { x: self.x, y: self.y+1, tool: self.tool }, 1 ),
            ( State { x: self.x, y: self.y, tool: None }, 7 ),
            ( State { x: self.x, y: self.y, tool: Torch }, 7 ),
            ( State { x: self.x, y: self.y, tool: Gear }, 7 ),
        ]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct SearchState {
    actual_dist: u64,
    min_total: u64,
    state: State
}

impl Ord for SearchState {
    fn cmp(&self, other: &SearchState) -> Ordering {
        other.min_total.cmp(&self.min_total).then_with(|| self.actual_dist.cmp(&other.actual_dist))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &SearchState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



fn main() {
    //let depth = 510;
    //let target=(10,10);
    let depth = 10914;
    let target = (9,739);
    let width = target.0 + BUFFER + 1;
    let height = target.1 + BUFFER + 1;
    let mut indexes : Vec<Vec<u64>> = (0..height).map(|_| {
        vec![0; width]
    }).collect();
    for x in 1..width {
        indexes[0][x] = (x as u64 * 16807 + depth) % MODULO;
    }
    for y in 1..height {
        indexes[y][0] = (y as u64 * 48271 + depth) % MODULO;
    }
    for y0 in 2..(width+height) {
        let offset_start = if y0 > height-1 {
            y0 - (height - 1)
        } else {
            1
        };
        let offset_end = if y0 > width-1 {
            width - 1
        } else {
            y0 - 1
        };
        //println!("{} -> {}..{}", y0, offset_start,offset_end);
        for offset in offset_start..offset_end+1 {
            indexes[y0-offset][offset] = (indexes[y0-offset][offset-1] * indexes[y0-offset-1][offset] + depth) % MODULO;
            //println!("i[{}][{}]",y0-offset,offset);
        }
    }
    indexes[target.1][target.0] = 0;
    /*for row in indexes.iter() {
        for cell in row.iter() {
            print!("{}", ['.','=','|'][(cell % 3) as usize]);
        };
        println!("");
    }*/
    

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    // -(dist+heur), -dist, state
    queue.push( SearchState { actual_dist: 0, min_total: (target.0 + target.1) as u64, state: State { x: 0, y: 0, tool: Torch } } );
    while let Some(cur) = queue.pop() {
        let state = cur.state;
        if (state.x, state.y, state.tool) == (target.0 as u64, target.1 as u64, Torch) {
            println!("{:?}", cur);
            break;
        }
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);
        //println!("explore {:?}", cur);

        for (neighbor, cost) in state.neighbors().iter() {
            let terrain = indexes[neighbor.y as usize][neighbor.x as usize] % 3;
            let compatible = match neighbor.tool {
                None => terrain != 0,
                Torch => terrain != 1,
                Gear => terrain != 2,
            };
            if !visited.contains(neighbor) && compatible {
                let heur_remain = ((neighbor.y as i64 - target.1 as i64).abs() + (neighbor.x as i64 - target.0 as i64).abs() + if neighbor.tool == Torch { 0 } else { 7 }) as u64;
                //println!("  n: {} {} {:?}", cur.actual_dist + cost, cur.actual_dist + cost + heur_remain, neighbor);
                queue.push( SearchState { actual_dist: cur.actual_dist + cost, min_total: cur.actual_dist + cost + heur_remain, state: *neighbor });
            }
        }
    }
    /*let mut v : Vec<State> = visited.into_iter().collect();
    v.sort_by_key(|s| (s.y + s.x, s.y, s.x));
    for vv in v { println!("{:?}", vv)};*/
}
