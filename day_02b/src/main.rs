use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();
    let lines : Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    for (i, l1) in lines.iter().enumerate() {
        for l2 in lines[i+1..].iter() {
            let diffs = l1.chars().zip(l2.chars()).filter(|&(c1,c2)| c1!=c2);
            if diffs.count() == 1 {
                let sims = l1.chars().zip(l2.chars()).filter(|&(c1,c2)| c1 == c2);
                let word : String = sims.map(|(c1,_)| c1).collect();
                println!("{}", word);
                return;
            }
        }
    }
}
