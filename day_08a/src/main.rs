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
    //let all_metadata =
    let mut sum = 0;
    for _ in 0..*child_count {
        sum += visit(nums);
    }
    for _ in 0..*metadata_count {
        sum += nums.next().unwrap();
    }
    sum
}