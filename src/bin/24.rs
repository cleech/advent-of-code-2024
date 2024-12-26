use rustc_hash::FxHashMap as HashMap;
use sscanf::scanf;

advent_of_code::solution!(24);

enum GateType {
    AND,
    OR,
    XOR,
}
use GateType::*;

struct Gate {
    op: GateType,
    a: String,
    b: String,
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

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
