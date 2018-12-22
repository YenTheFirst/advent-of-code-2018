use std::io::stdin;
use std::collections::{HashSet, HashMap};
use std::str::Chars;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut map = HashMap::new();
    follow_regex(&mut input.chars(), &mut map, vec![(0,0)].into_iter().collect());
    /*println!("map: {:?}", map);
    println!("count: {}", map.len());*/

    let mut visited = HashMap::new();
    visited.insert((0,0), 0);
    let mut cur : HashSet<(i32,i32)> = vec![(0,0)].into_iter().collect();
    let mut d = 1;
    loop {
        let next : HashSet<(i32,i32)> = cur.iter().flat_map(|h| map[h].iter()).filter(|h| !visited.contains_key(h)).cloned().collect();
        next.iter().for_each(|&h| {visited.insert(h,d);});
        if next.is_empty() {
            break;
        }
        d += 1;
        cur = next;
    }
    
    println!("{}", d);
    println!("{}", visited.values().max().unwrap());
    println!("{}", visited.values().filter(|&&c| c>=1000).count());
}
/*
fn print_map(map: &HashMap<(i32,i32)>, HashSet<(i32,i32)>) {

}*/

fn follow_regex(mut chars: &mut Chars, mut map: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>, start: HashSet<(i32,i32)>) -> HashSet<(i32,i32)> {
    let mut heads = start.clone();
    let mut return_heads = HashSet::new();
    while let Some(ch) = chars.next() {
        match ch {
            '^' | '$' | '\n' => (),
            ')' => {return_heads.extend(heads); return return_heads;},
            '(' => {heads = follow_regex(&mut chars, &mut map, heads)}
            '|' => {return_heads.extend(heads); heads = start.clone();},
            'N' => { heads = do_step(&mut map, &heads, (0,1)) },
            'S' => { heads = do_step(&mut map, &heads, (0,-1)) },
            'E' => { heads = do_step(&mut map, &heads, (1,0)) },
            'W' => { heads = do_step(&mut map, &heads, (-1,0)) },
            x => unreachable!("char = {}",x)
        }
    }
    return return_heads;
}

fn do_step(map: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>, heads: &HashSet<(i32, i32)>, step: (i32, i32)) -> HashSet<(i32, i32)> {
    heads.iter().for_each(|&h| {
        let n = (h.0+step.0, h.1+step.1);
        map.entry(h).or_insert(HashSet::new()).insert(n);
        map.entry(n).or_insert(HashSet::new()).insert(h);
    });
    heads.iter().map(|h| (h.0+step.0, h.1+step.1)).collect()
}
