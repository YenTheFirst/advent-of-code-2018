use std::env::args;
fn main() {
    let mut a = args();
    a.next();
    let serial = a.next().unwrap().parse::<i32>().unwrap();

    let grid : Vec<Vec<i32>> = std::iter::once(vec![0;301]).chain((1..300+1).map(|y| {
        std::iter::once(0).chain((1..300+1).map(move |x| {
            let rack_id = x+10;
            let p = ((rack_id * y) + serial) * rack_id;
            (p / 100) % 10 - 5
        })).collect()
    })).collect();

    let sums : Vec<Vec<i32>> = grid.iter().map(|row| {
        row.iter().scan(0, |sum, cell| {
            *sum = *sum + cell;
            Some(*sum)
        })
    }).scan(vec![0;301], |sumrow, row| {
        *sumrow = sumrow.iter().zip(row).map(|(a,b)| a+b).collect();
        Some(sumrow.clone())
    }).collect();

    let (x,y,n) = (1..300+1).flat_map(|n| {
        (1..300-n+1).flat_map(move |y| {
            (1..300-n+1).map(move |x| {
                (x,y,n)
            })
        })
    }).max_by_key(|&(x,y,n)| {
        sums[y+n-1][x+n-1] - sums[y+n-1][x-1] - sums[y-1][x+n-1] + sums[y-1][x-1]
    }).unwrap();

    println!("{},{},{}", x,y,n);
}
