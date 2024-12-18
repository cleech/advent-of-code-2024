advent_of_code::solution!(17);

use itertools::Itertools;
use sscanf::scanf;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Cpu {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    program: Vec<u8>,
    output: VecDeque<u8>,
}

#[derive(Debug)]
enum Opcode {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc,
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}
use Opcode::*;

impl Cpu {
    fn decode(opcode: u8, operand: u8) -> Opcode {
        match opcode {
            0 => Adv(operand),
            1 => Bxl(operand),
            2 => Bst(operand),
            3 => Jnz(operand),
            4 => Bxc,
            5 => Out(operand),
            6 => Bdv(operand),
            7 => Cdv(operand),
            _ => unreachable!(),
        }
    }
    fn combo(&self, operand: u8) -> Option<usize> {
        match operand {
            n @ 0..=3 => Some(n.into()),
            4 => Some(self.a),
            5 => Some(self.b),
            6 => Some(self.c),
            7 => None,
            _ => unreachable!(),
        }
    }
    fn process(&mut self) -> Option<()> {
        let opcode = Self::decode(self.program[self.ip], self.program[self.ip + 1]);
        // println!("{:?}", opcode);
        match opcode {
            Adv(combo) => self.a >>= self.combo(combo)?,
            Bxl(literal) => self.b ^= literal as usize,
            Bst(combo) => self.b = self.combo(combo)? % 8,
            Jnz(literal) => {
                if self.a != 0 {
                    self.ip = literal as usize;
                    return Some(());
                }
            }
            Bxc => self.b ^= self.c,
            Out(combo) => {
                let out = (self.combo(combo)? % 8).try_into().unwrap();
                self.output.push_back(out);
            }
            Bdv(combo) => self.b = self.a >> self.combo(combo)?,
            Cdv(combo) => self.c = self.a >> self.combo(combo)?,
        }
        self.ip += 2;
        Some(())
    }
}

fn parse(input: &str) -> Option<Cpu> {
    let (prefix, suffix) = input.split_once("\n\n")?;
    let mut registers = prefix.lines();
    let a = scanf!(registers.next()?, "Register A: {usize}").ok()?;
    let b = scanf!(registers.next()?, "Register B: {usize}").ok()?;
    let c = scanf!(registers.next()?, "Register C: {usize}").ok()?;
    let suffix = suffix.trim();
    let p = scanf!(suffix, "Program: {}", str).unwrap();
    let p = p
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    Some(Cpu {
        a,
        b,
        c,
        ip: 0,
        program: p,
        output: VecDeque::default(),
    })
}

pub fn part_one(input: &str) -> Option<String> {
    let mut cpu = parse(input)?;
    while cpu.ip < cpu.program.len() - 1 {
        cpu.process()?;
    }
    // println!("{:?}", cpu.output);
    Some(cpu.output.into_iter().map(|n| n.to_string()).join(","))
}

pub fn part_two(input: &str) -> Option<usize> {
    let cpu = parse(input)?;
    for a in 0..usize::MAX {
        let mut cpu = cpu.clone();
        cpu.a = a;
        while cpu.ip < cpu.program.len() - 1 {
            if !cpu.process().is_some() {
                break;
            }
            if cpu.output.len() > cpu.program.len() {
                break;
            }
        }
        let output: Vec<u8> = cpu.output.into();

        if cpu.program.len() == output.len() {
            let o = output.iter().map(|n| n.to_string()).join(",");
            let p = cpu.program.iter().map(|n| n.to_string()).join(",");
            if o == p {
                println!("({}) o: {}", a, o);
                println!("({}) p: {}", a, p);
                return Some(a);
            }
        }
    }
    // println!("{:?}", cpu.output);
    None
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
