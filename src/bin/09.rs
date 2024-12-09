advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Free,
    Used(usize),
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut v = input
        .as_bytes()
        .chunks(2)
        .enumerate()
        .flat_map(|(id, file)| {
            let used = file.get(0);
            let free = file.get(1);
            let mut v = Vec::new();
            if let Some(n) = used {
                let n = *n - b'0';
                v.append(&mut [Block::Used(id)].repeat(n.into()));
            }
            if let Some(n) = free {
                if *n >= b'0' {
                    let n = *n - b'0';
                    v.append(&mut [Block::Free].repeat(n.into()));
                }
            }
            v
        })
        .collect::<Vec<Block>>();
    let mut i = 0;
    for j in (0..v.len()).rev() {
        if v[j] == Block::Free {
            continue;
        }
        loop {
            if v[i] != Block::Free {
                i += 1;
            } else {
                break;
            }
        }
        if i > j {
            break;
        }
        v.swap(i, j);
    }
    let cs = v.iter().enumerate().fold(0, |cs, (n, id)| {
        if let Block::Used(id) = id {
            cs + n * (*id)
        } else {
            cs
        }
    });
    Some(cs)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Extent {
    Free(usize),
    Used { id: usize, len: usize, moved: bool },
}

fn printit(v: &Vec<Extent>) {
    println!(
        "{:?}",
        v.iter()
            .map(|&e| match e {
                Extent::Free(n) => ".".repeat(n).chars().collect::<String>(),
                Extent::Used {
                    id,
                    len: n,
                    moved: _,
                } => id.to_string().repeat(n).chars().collect(),
            })
            .collect::<String>()
    );
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut v = input
        .as_bytes()
        .chunks(2)
        .enumerate()
        .flat_map(|(id, file)| {
            let used = file.get(0);
            let free = file.get(1);
            let mut v = Vec::new();
            if let Some(n) = used {
                v.push(Extent::Used {
                    id,
                    len: (*n - b'0') as usize,
                    moved: false,
                });
            }
            if let Some(n) = free {
                if *n >= b'0' {
                    let n: usize = (*n - b'0').into();
                    if n > 0 {
                        v.push(Extent::Free(n));
                    }
                }
            }
            v
        })
        .collect::<Vec<Extent>>();

    // printit(&v);
    let mut added = 0;
    for ref mut j in (0..v.len()).rev() {
        let extlen;
        let extid;

        let mut j = *j + added;
        match v[j] {
            Extent::Free(_) => continue,
            Extent::Used { id, len, moved } => {
                if moved {
                    continue;
                }
                extlen = len;
                extid = id;
            }
        }

        //         println!("looking to move: {extid}");

        for i in 0..v.len() {
            match v[i] {
                Extent::Used { .. } => {
                    continue;
                }
                Extent::Free(len) => {
                    if i >= j {
                        break;
                    }
                    if len < extlen {
                        continue;
                    }
                    if extlen < len {
                        v[i] = Extent::Free(extlen);
                        v.insert(i + 1, Extent::Free(len - extlen));
                        j += 1;
                        added += 1;
                    }
                    v[j] = Extent::Used {
                        id: extid,
                        len: extlen,
                        moved: true,
                    };
                    v.swap(i, j);
                    // printit(&v);
                    break;
                }
            }
        }
    }
    let cs = v.iter().fold((0, 0), |(cs, off), e| {
        if let &Extent::Used { id, len, .. } = e {
            (
                cs + (off..(off + len))
                    .into_iter()
                    .map(|x| x * id)
                    .sum::<usize>(),
                off + len,
            )
        } else {
            if let &Extent::Free(len) = e {
                (cs, off + len)
            } else {
                panic!()
            }
        }
    });
    Some(cs.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
