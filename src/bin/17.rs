advent_of_code::solution!(17);

use itertools::Itertools;
use sscanf::scanf;
use std::{ops::ControlFlow, rc::Rc};

#[derive(Debug, Clone)]
struct Cpu {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    program: Rc<Vec<u8>>,
}

#[derive(Debug)]
enum Opcode {
    Adv(u64),
    Bxl(u64),
    Bst(u64),
    Jnz(u64),
    Bxc,
    Out(u64),
    Bdv(u64),
    Cdv(u64),
}
use Opcode::*;

impl Cpu {
    fn run_one(&mut self) -> Option<u8> {
        let literal = |operand: u8| -> u64 { operand.into() };
        let combo = |operand: u8| -> u64 {
            match operand {
                n @ 0..=3 => n.into(),
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => unreachable!(),
            }
        };
        let decode = |opcode: u8, operand: u8| -> Opcode {
            match opcode {
                0 => Adv(combo(operand)),
                1 => Bxl(literal(operand)),
                2 => Bst(combo(operand)),
                3 => Jnz(literal(operand)),
                4 => Bxc,
                5 => Out(combo(operand)),
                6 => Bdv(combo(operand)),
                7 => Cdv(combo(operand)),
                _ => unreachable!(),
            }
        };
        let opcode = decode(self.program[self.ip], self.program[self.ip + 1]);
        match opcode {
            Adv(op) => self.a >>= op,
            Bxl(op) => self.b ^= op,
            Bst(op) => self.b = op % 8,
            Jnz(op) => {
                if self.a != 0 {
                    self.ip = op as usize;
                    return None;
                }
            }
            Bxc => self.b ^= self.c,
            Out(op) => {
                let out = (op % 8).try_into().unwrap();
                self.ip += 2;
                return Some(out);
            }
            Bdv(op) => self.b = self.a >> op,
            Cdv(op) => self.c = self.a >> op,
        }
        self.ip += 2;
        None
    }
    fn run_to_next_output(&mut self) -> Option<u8> {
        while self.ip < self.program.len() - 1 {
            let out = self.run_one();
            if out.is_some() {
                return out;
            }
        }
        None
    }
    fn run_to_completion(&mut self) -> Vec<u8> {
        let mut output = vec![];
        while let Some(out) = self.run_to_next_output() {
            output.push(out);
        }
        output
    }
}

fn parse(input: &str) -> Option<Cpu> {
    let (prefix, suffix) = input.split_once("\n\n")?;
    let mut registers = prefix.lines();
    let a = scanf!(registers.next()?, "Register A: {u64}").ok()?;
    let b = scanf!(registers.next()?, "Register B: {u64}").ok()?;
    let c = scanf!(registers.next()?, "Register C: {u64}").ok()?;
    let suffix = suffix.trim();
    let p = scanf!(suffix, "Program: {}", str).unwrap();
    let p = Rc::new(
        p.split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<_>>(),
    );
    Some(Cpu {
        a,
        b,
        c,
        ip: 0,
        program: p,
    })
}

pub fn part_one(input: &str) -> Option<String> {
    let mut cpu = parse(input)?;
    let output = cpu.run_to_completion();
    Some(output.into_iter().map(|n| n.to_string()).join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    // All of these functions operate on an input in register A, and loop until
    // it's reduced to 0. Each loop outputs a 3-bit value, derivied from A.
    // Knowing that A is reduced by 3-bits each pass, and that the low 3-bits
    // determine the output, we can solve this 3-bits at a time.
    let cpu = parse(input)?;
    _part_two(&cpu, cpu.program.len(), 0).break_value()
}

fn _part_two(cpu: &Cpu, depth: usize, a: u64) -> ControlFlow<u64> {
    if depth == 0 {
        return ControlFlow::Break(a);
    }
    for rem in 0..8 {
        let mut c = cpu.clone();
        c.a = (a << 3) | rem;
        let o = c.run_to_next_output().unwrap();
        if o == cpu.program[depth - 1] {
            _part_two(cpu, depth - 1, (a << 3) | rem)?
        }
    }
    ControlFlow::Continue(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
