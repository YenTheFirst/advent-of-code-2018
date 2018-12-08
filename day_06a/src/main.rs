use std::io::{stdin,BufRead};

fn main() {
    let stdin = stdin();
    let coords : Vec<(i32,i32)> = stdin.lock().lines().map(|l| {
        let line = l.unwrap();
        let mut cs = line.split(", ");
        (cs.next().unwrap().parse::<i32>().unwrap(), cs.next().unwrap().parse::<i32>().unwrap() )
    }).collect();

    let min_x = coords.iter().map(|(x,_)| x).min().unwrap();
    let max_x = coords.iter().map(|(x,_)| x).max().unwrap();
    let min_y = coords.iter().map(|(_,y)| y).min().unwrap();
    let max_y = coords.iter().map(|(_,y)| y).max().unwrap();

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let trans_coords : Vec<(i32,i32)> = coords.iter().map(|(x,y)| (x-min_x, y-min_y)).collect();

    //I mean you could probably do this by generating lines of equal distance, and deriving a polygon, determining open/closed
    //but eh, rasterization should work well enough for now.
    let field : Vec<Option<usize>> = (0..height).flat_map(|y| {
        (0..width).map(|x| {
            let mut dists : Vec<(i32, usize)> = trans_coords.iter().enumerate().map(|(i, (cx, cy))| {
                ((x - cx).abs() + (y - cy).abs(), i)
            }).collect();
            dists.sort();

            if dists[0].0 == dists[1].0 {
                None
            } else {
                Some(dists[0].1)
            }
        }).collect::<Vec<Option<usize>>>().into_iter()
    }).collect();

    let mut borders : Vec<Option<usize>> = (0..width)
        .chain((0..width).map(|x| (height-1)*width + x))
        .chain((0..height).map(|y| y*width))
        .chain((0..height).map(|y| y*width + width-1))
        .map(|i| field[i as usize])
        .collect();
    borders.sort();
    borders.dedup();

    let biggest = (0..coords.len())
        .map(|i| Some(i))
        .filter(|i| borders.iter().find(|&b| b == i).is_none())
        .map(|i| field.iter().filter(|&&e| e==i).count())
        .max().unwrap();
    println!("{}", biggest);
}
