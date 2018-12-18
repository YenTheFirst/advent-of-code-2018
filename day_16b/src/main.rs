use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines : Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();

    let mut opcode_map : Vec<Vec<Opcode>> = Vec::new();
    for _ in 0..16 {
        use self::Opcode::*;
        opcode_map.push(vec![Addr, Addi, Mulr, Muli, Bani, Banr, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr]);
    }
    
    lines.chunks_exact(4).take_while(|chunk| chunk[0].starts_with("Before")).for_each(|chunk| {
        let before : Reg = chunk[0].split(|ch: char| !ch.is_numeric()).filter_map(|p| p.parse::<i32>().ok()).collect();
        let instr : Vec<i32> = chunk[1].split(|ch: char| !ch.is_numeric()).filter_map(|p| p.parse::<i32>().ok()).collect();
        let after : Reg = chunk[2].split(|ch: char| !ch.is_numeric()).filter_map(|p| p.parse::<i32>().ok()).collect();
        let i = instr[0] as usize;
        let (left, opm_and_right) = opcode_map.split_at_mut(i);
        let (opm_slice, right) = opm_and_right.split_at_mut(1);
        let opm = &mut opm_slice[0];
        let before_c = opm.len();
        opm.retain(|op| {
            op.call(instr[1], instr[2], instr[3], &before) == after
        });
        println!("op {} has {}->{} possibilities ({:?})", instr[0], before_c, opm.len(), opm);
        if opm.len() == 1 {
            left.iter_mut().chain(right.iter_mut()).for_each(|opm2| {
                if let Some(p) = opm2.iter().position(|el| *el == opm[0]) {
                    opm2.remove(p);
                }
            });
        }
        
    });
    if !opcode_map.iter().all(|ops| ops.len() == 1) {
        panic!("couldn't solve map: {:?}", opcode_map);
    }
    let start = lines.iter().rposition(|l| l=="").unwrap();
    let mut reg = vec![0,0,0,0]; //not actually stated in the description, so I hope that's the assumption...
    for l in lines[start+1..lines.len()].iter() {
        let instr : Vec<i32> = l.split(|ch: char| !ch.is_numeric()).filter_map(|p| p.parse::<i32>().ok()).collect();
        reg = opcode_map[instr[0] as usize][0].call(instr[1], instr[2], instr[3], &reg)
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

type Reg = Vec<i32>; // should really be a [i32, 4], but I just don't feel like fighting the compiler

impl Opcode {
    fn call(&self, a: i32, b: i32, c: i32, reg: &Reg) -> Reg {
        //use Opcode::*;
        let ai = a as usize;
        let bi = b as usize;
        let mut new_reg = reg.clone();
        new_reg[c as usize] = match *self {
            Opcode::Addr => reg[ai] + reg[bi],
            Opcode::Addi => reg[ai] + b,
            Opcode::Mulr => reg[ai] * reg[bi],
            Opcode::Muli => reg[ai] * b,
            Opcode::Banr => reg[ai] & reg[bi],
            Opcode::Bani => reg[ai] & b,
            Opcode::Borr => (reg[ai] | reg[bi]),
            Opcode::Bori => (reg[ai] | b),
            Opcode::Setr => reg[ai],
            Opcode::Seti => a,
            Opcode::Gtir => if a > reg[bi] { 1 } else { 0 },
            Opcode::Gtri => if reg[ai] > b { 1 } else { 0 },
            Opcode::Gtrr => if reg[ai] > reg[bi] { 1 } else { 0 },
            Opcode::Eqir => if a == reg[bi] { 1 } else { 0 },
            Opcode::Eqri => if reg[ai] == b { 1 } else { 0 },
            Opcode::Eqrr => if reg[ai] == reg[bi] { 1 } else { 0 },
        };
        new_reg
    }
}
