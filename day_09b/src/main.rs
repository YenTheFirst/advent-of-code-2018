use std::env::args;
use std::collections::LinkedList;

fn main() {
    let mut a = args();
    a.next();
    let num_players = a.next().unwrap().parse::<usize>().unwrap();
    let num_marble = a.next().unwrap().parse::<usize>().unwrap();

    let mut player_score = vec![0; num_players];
    let mut board = LinkedList::new();
    board.push_front(0);

    for marble in 1..num_marble+1 {
        if marble % 23 != 0 {
            let e = board.pop_front().unwrap();
            board.push_back(e);
            board.push_back(marble);
        } else {
            for _ in 0..7 {
                let e = board.pop_back().unwrap();
                board.push_front(e);
            }
            player_score[(marble % num_players)] += marble + board.pop_back().unwrap().clone();
            let e = board.pop_front().unwrap();
            board.push_back(e);
            //println!("{} {:?}", marble, player_score);
        }
    }
    println!("{} {}", player_score.iter().max().unwrap(), player_score.iter().fold(0, |s,x| s+x));
}
