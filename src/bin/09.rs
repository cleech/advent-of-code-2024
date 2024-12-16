advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Free,
    Used(usize),
}

fn checksum(fid: usize, off: usize, len: usize) -> usize {
    let last = off + len - 1;
    let n = (last * (last + 1)) / 2;
    let m = if off != 0 { (off * (off - 1)) / 2 } else { 0 };
    fid * (n - m)
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input
        .trim()
        .bytes()
        .map(|b| (b - b'0') as usize)
        .collect::<Vec<_>>();

    let mut left = 0;
    let mut right = map.len() - 1;

    let mut lba = 0;
    let mut cs = 0;

    let mut needed = map[right];

    while left < right {
        let n = map[left];
        let fid = left / 2;
        cs += checksum(fid, lba, n);
        lba += n;

        left += 1;
        let mut free_space = map[left];
        left += 1;

        while free_space > 0 {
            if needed == 0 {
                if left > right {
                    break;
                }
                right -= 2;
                needed = map[right];
            }
            let m = needed.min(free_space);
            let fid = right / 2;
            cs += checksum(fid, lba, m);
            lba += m;
            free_space -= m;
            needed -= m;
        }
    }
    cs += checksum(right / 2, lba, needed);
    Some(cs)
}

pub fn part_one_0(input: &str) -> Option<usize> {
    let mut v = input
        .trim()
        .as_bytes()
        .chunks(2)
        .enumerate()
        .flat_map(|(id, file)| {
            let used = file.first();
            let free = file.get(1);
            let mut v = Vec::new();
            if let Some(n) = used {
                let n = *n - b'0';
                v.append(&mut [Block::Used(id)].repeat(n.into()));
            }
            if let Some(n) = free {
                let n = *n - b'0';
                v.append(&mut [Block::Free].repeat(n.into()));
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

pub fn part_two(input: &str) -> Option<usize> {
    let mut v = input
        .trim()
        .as_bytes()
        .chunks(2)
        .enumerate()
        .flat_map(|(id, file)| {
            let used = file.first();
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
                let n: usize = (*n - b'0').into();
                if n > 0 {
                    v.push(Extent::Free(n));
                }
            }
            v
        })
        .collect::<Vec<Extent>>();

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
                    break;
                }
            }
        }
    }
    let cs = v.iter().fold((0, 0), |(cs, off), e| match *e {
        Extent::Used { id, len, .. } => (
            cs + (off..(off + len)).map(|x| x * id).sum::<usize>(),
            off + len,
        ),
        Extent::Free(len) => (cs, off + len),
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
