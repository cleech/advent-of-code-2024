use rustc_hash::FxHashMap as HashMap;
use sscanf::scanf;

advent_of_code::solution!(24);

#[derive(Debug, PartialEq, Eq)]
enum GateType {
    AND,
    OR,
    XOR,
}
use GateType::*;

#[derive(Debug)]
struct Gate {
    op: GateType,
    a: String,
    b: String,
    s: String,
}

fn parse(input: &str) -> Option<(HashMap<String, u8>, HashMap<String, Gate>)> {
    let (prefix, suffix) = input.split_once("\n\n")?;

    let mut wires = HashMap::default();
    for line in prefix.lines() {
        let (name, value) = line.split_once(": ")?;
        let name = name.to_string();
        let value = value.parse::<u8>().ok()?;
        wires.insert(name, value);
    }

    let mut gates = HashMap::default();
    for line in suffix.lines() {
        let (a, op, b, output) = scanf!(line, "{str} {str} {str} -> {str}").ok()?;
        let a = a.to_string();
        let b = b.to_string();
        let output = output.to_string();
        let gate = Gate {
            a,
            b,
            s: output.clone(),
            op: match op {
                "AND" => AND,
                "OR" => OR,
                "XOR" => XOR,
                _ => unreachable!(),
            },
        };
        gates.insert(output, gate);
    }
    Some((wires, gates))
}

fn eval(wires: &mut HashMap<String, u8>, gates: &HashMap<String, Gate>, output: &str) -> u8 {
    if let Some(value) = wires.get(output) {
        return *value;
    }
    if let Some(gate) = gates.get(output) {
        let a = eval(wires, gates, &gate.a);
        let b = eval(wires, gates, &gate.b);
        let value = match gate.op {
            AND => a & b,
            OR => a | b,
            XOR => a ^ b,
        };
        wires.insert(output.to_string(), value);
        return value;
    }
    unreachable!();
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut wires, gates) = parse(input)?;
    let mut outputs = gates
        .keys()
        .filter(|g| g.starts_with('z'))
        .collect::<Vec<_>>();
    outputs.sort();
    let mut solution: u64 = 0;
    for z in outputs.iter().rev() {
        eval(&mut wires, &gates, *z);
        solution <<= 1;
        solution += *wires.get(*z)? as u64;
    }
    Some(solution)
}

fn _check(wires: &mut HashMap<String, u8>, gates: &HashMap<String, Gate>, output: &str) -> u8 {
    if let Some(value) = wires.get(output) {
        return *value;
    }
    if let Some(gate) = gates.get(output) {
        let a = _check(wires, gates, &gate.a);
        let b = _check(wires, gates, &gate.b);
        let value = match gate.op {
            AND => a & b,
            OR => a | b,
            XOR => a ^ b,
        };
        wires.insert(output.to_string(), value);
        return value;
    }
    unreachable!();
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut _wires, gates) = parse(input)?;

    let half_addr = |n: usize| -> Option<()> {
        let s = format!("z{n:02}");
        let gate = gates.get(&s)?;

        println!("{gate:?}");

        assert!(gate.op == XOR);
        // only expect a single half adder on z00
        assert!(gate.a == "y00" || gate.a == "x00");
        assert!(gate.b == "x00" || gate.b == "y00");
        assert!(gate.s == "z00");
        // Cout will be checked from z01 Cin

        Some(())
    };

    let carry_out = |s: &str| -> Vec<String> {
        let mut output = vec![];
        let gate = &gates[s];
        assert!(gate.op == OR);
        let a = &gates[&gate.a];
        let b = &gates[&gate.b];
        if !(a.op == AND) {
            println!("!!! error at {:02}", a.s);
            output.push(a.s.clone());
        }
        if !(b.op == AND) {
            println!("!!! error at {:02}", b.s);
            output.push(b.s.clone());
        }
        output
    };

    let full_addr = |n: usize| -> Vec<String> {
        let mut output = vec![];
        let s = format!("z{n:02}");
        let gate = &gates[&s];

        println!("{gate:?}");

        assert!(gate.s == s);
        // assert!(gate.op == XOR);
        if !(gate.op == XOR) {
            println!("!!! error at z{n:02}");
            output.push(gate.s.clone());
            return output;
        }
        let a = &gates[&gate.a];
        let b = &gates[&gate.b];
        for input in &[a, b] {
            println!("{input:?}");
            let x = format!("x{n:02}");
            let y = format!("y{n:02}");
            if input.op == XOR {
                // assert!(input.a == x || input.a == y);
                if !(input.a == x || input.a == y) || !(input.b == x || input.b == y) {
                    println!("!!! error at {}", input.s);
                    output.push(input.s.clone());
                    // return None;
                }
            } else if input.op == AND {
                // Cout from half addr
                if !(input.a == "x00" || input.a == "y00")
                    || !(input.b == "x00" || input.b == "y00")
                {
                    println!("!!! error at {}", input.s);
                    output.push(input.s.clone());
                    // return None;
                }
            } else if input.op == OR {
                // Cout from full addr
                let cout = input;
                output.extend(carry_out(&cout.s));
            }
        }
        output
    };

    let mut output = vec![];

    // bit-0 half adder
    println!("== 00 ==");
    half_addr(0);

    // bits 1-44 full adder
    for n in 1..=44 {
        println!("== {n:02} ==");
        output.extend(full_addr(n));
    }
    // bit-45 carry out / overflow
    //

    output.sort();
    Some(output.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
