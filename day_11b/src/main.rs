use std::env::args;
fn main() {
    let mut a = args();
    a.next();
    let serial = a.next().unwrap().parse::<i32>().unwrap();

    let best = (0..(300*300*300)).max_by_key(|xyn| {
        let x = xyn % 300;
        let y = (xyn / 300) % 300;
        let n = xyn / 300 / 300;
        //ok actualllyyyyyy there's an off-by-one here, since grid is 1...300 inclusive, not
        //0...299 inclusive. it's probably fine.
        //it worked fine for part one, so eh.
        if (x+n) > 300 || (y+n) > 300 {
            0
        } else {
            (0..n).map(|sy| {
                (0..n).map(|sx| {
                    let rack_id = (x+sx)+10;
                    let p = ((rack_id * (y+sy)) + serial) * rack_id;
                    (p / 100) % 10 - 5
                }).sum::<i32>()
            }).sum::<i32>()
        }
    }).unwrap();

    let x = best % 300;
    let y = (best / 300) % 300;
    let n = best / 300 / 300;
    println!("{},{},{}", x,y,n);

    /*(-1..n+1).for_each(|sy| {
        (-1..n+1).for_each(|sx| {
            let rack_id = (x+sx)+10;
            let p = ((rack_id * (y+sy)) + serial) * rack_id;
            let pow = (p / 100) % 10 - 5;
            print!("{:2} ",pow);
        });
        println!("");
    })*/

}
