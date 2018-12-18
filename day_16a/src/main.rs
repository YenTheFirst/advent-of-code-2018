use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines : Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();
    
    let c = lines.chunks_exact(4).take_while(|chunk| chunk[0].starts_with("Before")).filter(|chunk| {
        let before : Reg = chunk[0].split(|ch: char| !ch.is_numeric()).filter_map(|p| p.parse::<i32>().ok()).collect();
        let instr : Vec<i32> = chunk[1].split(|ch: char| !ch.is_numeric()).filter_map(|p| p.parse::<i32>().ok()).collect();
        let after : Reg = chunk[2].split(|ch: char| !ch.is_numeric()).filter_map(|p| p.parse::<i32>().ok()).collect();
        use self::Opcode::*;
        let c = [Addr, Addi, Mulr, Muli, Bani, Banr, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr].iter().filter(|op| {
            op.call(instr[1], instr[2], instr[3], &before) == after
        }).count();
        c >= 3
    }).count();

    println!("{}", c);
}

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
