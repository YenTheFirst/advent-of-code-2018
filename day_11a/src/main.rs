use std::env::args;
fn main() {
    let mut a = args();
    a.next();
    let serial = a.next().unwrap().parse::<i32>().unwrap();

    let best = (0..(300-3)*(300-3)).max_by_key(|xy| {
        let x = xy % (300-3);
        let y = xy / (300-3);
        (0..3).map(|sy| {
            (0..3).map(|sx| {
                let rack_id = (x+sx)+10;
                let p = ((rack_id * (y+sy)) + serial) * rack_id;
                (p / 100) % 10 - 5
            }).sum::<i32>()
        }).sum::<i32>()
    }).unwrap();

    let x = best % (300-3);
    let y = best / (300-3);
    println!("{},{}", x,y);

    (-1..4).for_each(|sy| {
        (-1..4).for_each(|sx| {
            let rack_id = (x+sx)+10;
            let p = ((rack_id * (y+sy)) + serial) * rack_id;
            let pow = (p / 100) % 10 - 5;
            print!("{:2} ",pow);
        });
        println!("");
    })

}
