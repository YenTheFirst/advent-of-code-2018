use std::io::BufRead;
use std::collections::HashSet;
use crate::Opcode::*;
fn main() {
    let stdin = std::io::stdin();
    let lines : Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();
    let mut seen : HashSet<i64> = HashSet::new();

    //let mut reg : Reg = vec![0,0,0,0,0,0];
    let mut reg : Reg = vec![0,0,0,0,0,0];
    let ip_reg = lines[0].split_at(4).1.parse::<usize>().unwrap();
    
    let program : Vec<(Opcode, Vec<i64>)> = lines[1..].iter().map(|line| {    
        let instr_nums : Vec<i64> = line.split(|ch: char| !ch.is_numeric()).filter_map(|p| p.parse::<i64>().ok()).collect();
        let instr = match line.split_at(4).0 {
            "addr" => Addr,
            "addi" => Addi,
            "mulr" => Mulr,
            "muli" => Muli,
            "bani" => Bani,
            "banr" => Banr,
            "borr" => Borr,
            "bori" => Bori,
            "setr" => Setr,
            "seti" => Seti,
            "gtir" => Gtir,
            "gtri" => Gtri,
            "gtrr" => Gtrr,
            "eqir" => Eqir,
            "eqri" => Eqri,
            "eqrr" => Eqrr,
            _ => unreachable!()
        };
        (instr, instr_nums)
    }).collect();
    while reg[ip_reg] >= 0 && reg[ip_reg] < program.len() as i64 {
        let (instr, nums) = &program[reg[ip_reg] as usize];
        let next_reg = instr.call(nums[0], nums[1], nums[2], &reg);
        if reg[ip_reg] == 28 {
            println!("ip={} {:?} {:?} {} {} {} {:?}", reg[ip_reg], reg, instr, nums[0], nums[1], nums[2], next_reg);
            if seen.contains(&reg[4]) {
                return;
            } else {
                seen.insert(reg[4]);
            }
        };
        reg = next_reg;
        reg[ip_reg] += 1;
    }
    println!("{:?}", reg);
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Bani,
    Banr,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr
}

type Reg = Vec<i64>; // should really be a [i64, 6], but I just don't feel like fighting the compiler

impl Opcode {
    fn call(&self, a: i64, b: i64, c: i64, reg: &Reg) -> Reg {
        let ai = a as usize;
        let bi = b as usize;
        let mut new_reg = reg.clone();
        new_reg[c as usize] = match *self {
            Addr => reg[ai] + reg[bi],
            Addi => reg[ai] + b,
            Mulr => reg[ai] * reg[bi],
            Muli => reg[ai] * b,
            Banr => reg[ai] & reg[bi],
            Bani => reg[ai] & b,
            Borr => (reg[ai] | reg[bi]),
            Bori => (reg[ai] | b),
            Setr => reg[ai],
            Seti => a,
            Gtir => if a > reg[bi] { 1 } else { 0 },
            Gtri => if reg[ai] > b { 1 } else { 0 },
            Gtrr => if reg[ai] > reg[bi] { 1 } else { 0 },
            Eqir => if a == reg[bi] { 1 } else { 0 },
            Eqri => if reg[ai] == b { 1 } else { 0 },
            Eqrr => if reg[ai] == reg[bi] { 1 } else { 0 },
        };
        new_reg
    }
}
