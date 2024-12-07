use std::{cmp::Ordering, collections::HashMap};

fn parse_input(s: &str) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let mut partital_ord: Vec<(u64, u64)> = vec![];
    let mut updates: Vec<Vec<u64>> = vec![];
    s.lines().for_each(|l| {
        if let Some((x, y)) = l.split_once('|') {
            partital_ord.push((x.parse().unwrap(), y.parse().unwrap()));
        } else if l.contains(',') {
            updates.push(l.split(',').map(|n| n.parse().unwrap()).collect());
        }
    });
    (partital_ord, updates)
}

fn is_partially_ordered(partial_ord: &HashMap<(u64, u64), Ordering>, update: &[u64]) -> bool {
    (0..update.len()).all(|i| {
        (i + 1..update.len()).all(|j| {
            partial_ord
                .get(&(update[i], update[j]))
                .is_some_and(|ord| *ord != Ordering::Greater)
        })
    })
}

pub fn process_part1(input: &str) -> u64 {
    let (partial_ord_pairs, updates) = parse_input(input);
    let partial_ord: HashMap<(u64, u64), Ordering> = partial_ord_pairs
        .iter()
        .flat_map(|(x, y)| vec![((*x, *y), Ordering::Less), ((*y, *x), Ordering::Greater)])
        .collect();
    updates
        .iter()
        .filter(|p| is_partially_ordered(&partial_ord, p))
        .map(|p| p.get(p.len() / 2).unwrap())
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    let (partial_ord_pairs, updates) = parse_input(input);
    let partial_ord: HashMap<(u64, u64), Ordering> = partial_ord_pairs
        .iter()
        .flat_map(|(x, y)| vec![((*x, *y), Ordering::Less), ((*y, *x), Ordering::Greater)])
        .collect();
    updates
        .iter()
        .filter(|p| !is_partially_ordered(&partial_ord, p))
        .map(|p| {
            let mut pp = p.clone();
            pp.sort_by(|x, y| *partial_ord.get(&(*x, *y)).unwrap_or(&Ordering::Equal));
            *pp.get(p.len() / 2).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), 143);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE), 123)
    }
}
