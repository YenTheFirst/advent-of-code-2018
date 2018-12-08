use std::io::{stdin,BufRead};
const MAX_DIST : i32 = 10_000;
fn main() {
    let stdin = stdin();
    let coords : Vec<(i32,i32)> = stdin.lock().lines().map(|l| {
        let line = l.unwrap();
        let mut cs = line.split(", ");
        (cs.next().unwrap().parse::<i32>().unwrap(), cs.next().unwrap().parse::<i32>().unwrap() )
    }).collect();

    let (xs, ys): (Vec<_>, Vec<_>) = coords.iter().cloned().unzip();
    //let mut xs : Vec<&i32> = coords.iter().map(|(x,_)| x).collect();
    //let mut ys : Vec<&i32> = coords.iter().map(|(_,y)| y).collect();
    let x_max = xs.iter().max().unwrap();
    let x_sum : i32 = xs.iter().sum();
    let x_len = xs.iter().len() as i32;
    let y_max = ys.iter().max().unwrap();
    let y_sum : i32 = ys.iter().sum();
    let y_len = ys.iter().len() as i32;


    let x_start : i32 = (MAX_DIST - x_sum) / - x_len;
    let x_end : i32 = (MAX_DIST - x_sum + 2*x_max*x_len) / x_len;
    let y_start : i32 = (MAX_DIST - y_sum) / - y_len;
    let y_end : i32 = (MAX_DIST - y_sum + 2*y_max*y_len) / y_len;

    let size = (y_start..y_end).flat_map(|y| {
        (x_start..x_end).map(|x| {
            coords.iter().map(|(cx,cy)| (x-cx).abs() + (y-cy).abs()).sum()
        }).collect::<Vec<i32>>().into_iter()
    }).filter(|&d| d < MAX_DIST).count();

    println!("{}", size)
}