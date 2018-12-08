use std::io::stdin;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("couldn't read input");
    let nums : Vec<_> = input.trim().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();

    let sum = visit(&mut nums.iter());
    println!("{:?}",sum);
}

fn visit(nums: &mut std::slice::Iter<u32>) -> u32 {
    let child_count = nums.next().unwrap();
    let metadata_count = nums.next().unwrap();
    println!("{} {}", child_count, metadata_count);
    //let all_metadata =
    
    if *child_count == 0 {
        
        let r = (0..*metadata_count).map (|_| {
            *nums.next().unwrap()
        }).sum();
        println!("{} self meta = {}", *metadata_count,r);
        r
    } else {
        let children : Vec<u32> = (0..*child_count).map(|_| visit(nums)).collect();
        println!("children: {:?}", children);
        let r = (0..*metadata_count).map (|_| {
            let i = *nums.next().unwrap() as usize;
            println!("i={}",i);
            children.get(i-1).unwrap_or(&0)
        }).sum();
        println!("{} collect meta = {}", *metadata_count,r);
        r
    }
}