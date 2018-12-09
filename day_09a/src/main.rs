use std::env::args;
fn main() {
    let mut a = args();
    a.next();
    let num_players = a.next().unwrap().parse::<usize>().unwrap();
    let num_marble = a.next().unwrap().parse::<usize>().unwrap();

    let mut player_score = vec![0; num_players];
    let mut board = vec![0; 1];

    for marble in 1..num_marble+1 {
        if marble % 23 != 0 {
            board.rotate_left(1);
            board.push(marble);
        } else {
            board.rotate_right(7);
            player_score[(marble % num_players)] += marble + board.pop().unwrap();
            board.rotate_left(1);
            //println!("{} {:?}", marble, player_score);
        }
    }
    println!("{} {}", player_score.iter().max().unwrap(), player_score.iter().fold(0, |s,x| s+x));
}
