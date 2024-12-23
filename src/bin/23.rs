use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u64> {
    let mut computers = HashSet::default();
    let mut links = HashMap::default();
    for line in input.lines() {
        let (a, b) = line.split_once('-')?;
        computers.insert(a);
        computers.insert(b);
        links
            .entry(a)
            .and_modify(|lan: &mut Vec<&str>| lan.push(b))
            .or_insert(vec![b]);
        links
            .entry(b)
            .and_modify(|lan: &mut Vec<&str>| lan.push(a))
            .or_insert(vec![a]);
    }

    let mut count = 0;
    for a in computers {
        let a_neigh = links.get(a)?;
        for &b in a_neigh {
            if b > a {
                let b_neigh = links.get(b)?;
                for &c in b_neigh {
                    if c > b
                        && a_neigh.contains(&c)
                        && (a.as_bytes()[0] == b't'
                            || b.as_bytes()[0] == b't'
                            || c.as_bytes()[0] == b't')
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    Some(count)
}

fn bron_kerbosch(
    graph: &HashMap<String, HashSet<String>>,
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    max_clique: &mut HashSet<String>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            *max_clique = r.clone();
        }
        return;
    }

    let pivot = p.union(&x).next().unwrap().clone();
    let neighbors = &graph[&pivot];
    let candidates: Vec<_> = p.difference(neighbors).cloned().collect();

    for v in candidates {
        let mut new_r = r.clone();
        new_r.insert(v.clone());
        let new_p: HashSet<_> = p.intersection(&graph[&v]).cloned().collect();
        let new_x: HashSet<_> = x.intersection(&graph[&v]).cloned().collect();

        bron_kerbosch(graph, new_r, new_p, new_x, max_clique);

        p.remove(&v);
        x.insert(v);
    }
}

fn find_max_clique(graph: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut max_clique = HashSet::default();
    let all_nodes: HashSet<_> = graph.keys().cloned().collect();
    bron_kerbosch(
        graph,
        HashSet::default(),
        all_nodes.clone(),
        HashSet::default(),
        &mut max_clique,
    );
    max_clique
}

pub fn part_two(input: &str) -> Option<String> {
    let mut graph = HashMap::default();
    for line in input.lines() {
        let (a, b) = line.split_once('-')?;
        graph
            .entry(a.to_string())
            .or_insert(HashSet::default())
            .insert(b.to_string());
        graph
            .entry(b.to_string())
            .or_insert(HashSet::default())
            .insert(a.to_string());
    }

    let clique = find_max_clique(&graph);
    let mut c = clique.into_iter().collect::<Vec<_>>();
    c.sort();
    Some(c[..].join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
