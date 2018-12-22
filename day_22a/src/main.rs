const MODULO : u64 = 20183;
fn main() {
    //let depth = 510;
    //let target=(10,10);
    let depth = 10914;
    let target = (9,739);
    let mut indexes : Vec<Vec<u64>> = (0..target.1+1).map(|_| {
        vec![0; target.0+1]
    }).collect();
    for x in 1..target.0+1 {
        indexes[0][x] = (x as u64 * 16807 + depth) % MODULO;
    }
    for y in 1..target.1+1 {
        indexes[y][0] = (y as u64 * 48271 + depth) % MODULO;
    }
    for y0 in 2..(target.0+target.1) {
        let offset_start = if y0 > target.1 {
            y0 - target.1
        } else {
            1
        };
        let offset_end = if y0 > target.0 {
            target.0
        } else {
            y0 - 1
        };
        //println!("{} -> {}..{}", y0, offset_start,offset_end);
        for offset in offset_start..offset_end+1 {
            indexes[y0-offset][offset] = (indexes[y0-offset][offset-1] * indexes[y0-offset-1][offset] + depth) % MODULO;
            //println!("i[{}][{}]",y0-offset,offset);
        }
    }
    println!("i[1][1] = {}", indexes[1][1]);

    for row in indexes.iter() {
        for cell in row.iter() {
            print!("{}", ['.','=','|'][(cell % 3) as usize]);
        };
        println!("");
    }
    let danger : u64 = indexes.iter().flat_map(|row| row.iter().map(|cell| cell % 3)).sum();
    println!("danger: {}", danger);
}
