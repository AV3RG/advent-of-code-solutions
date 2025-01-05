use std::collections::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let connections = input.trim().lines().map(|line| {
        let (p1, p2) = line.split_once("-").unwrap();
        return if p1 < p2 { (p1, p2) } else { (p2, p1) };
    }).sorted().collect_vec();
    let mut count = 0;
    for i in 0..connections.len() {
        for j in i+1..connections.len() {
            if connections[i].0 == connections[j].0 &&
                connections.contains(&(connections[i].1, connections[j].1)) &&
                (connections[i].0.starts_with("t") || connections[i].1.starts_with("t") || connections[j].1.starts_with("t"))
            {
                count += 1;
            }
        }
    }
    Some(count)
}

fn bron_kerbosch(
    r: &HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    g: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<Vec<String>>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > 2 {
            let mut clique: Vec<String> = r.iter().cloned().collect();
            clique.sort();
            cliques.push(clique);
        }
        return;
    }

    let pivot = p
        .union(x)
        .max_by_key(|v| g.get(*v).map_or(0, |neighbors| neighbors.len()))
        .cloned();

    if let Some(pivot_vertex) = pivot {
        let neighbors = g.get(&pivot_vertex).cloned().unwrap_or_default();
        let candidates: Vec<String> = p.difference(&neighbors).cloned().collect();

        for v in candidates {
            let mut new_r = r.clone();
            new_r.insert(v.clone());

            let neighbors_v = g.get(&v).cloned().unwrap_or_default();
            let mut new_p = p.intersection(&neighbors_v).cloned().collect::<HashSet<String>>();
            let mut new_x = x.intersection(&neighbors_v).cloned().collect::<HashSet<String>>();

            bron_kerbosch(&new_r, &mut new_p, &mut new_x, g, cliques);
            p.remove(&v);
            x.insert(v);
        }
    }
}
pub fn part_two(input: &str) -> Option<String> {
    let mut connections = HashMap::new();
    let mut keys = HashSet::new();
    input.trim().lines().for_each(|line| {
        let (p1, p2) = line.split_once("-").map(|(a, b)| (a.to_string(), b.to_string())).unwrap();
        connections.entry(p1.clone()).or_insert_with(HashSet::new).insert(p2.clone());
        connections.entry(p2.clone()).or_insert_with(HashSet::new).insert(p1.clone());
        keys.insert(p1.clone());
        keys.insert(p2.clone());
    });
    let mut cliques = Vec::new();
    bron_kerbosch(&mut HashSet::new(), &mut keys, &mut HashSet::new(), &connections, &mut cliques);
    cliques.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap().into_iter().sorted().join(",").into()
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
