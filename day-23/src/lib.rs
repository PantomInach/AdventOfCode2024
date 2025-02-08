use std::{char, collections::HashSet};

use itertools::Itertools;

const T: u8 = b't' - b'a';
type PC = (u8, u8);

struct AdjMatrix([[bool; 26 * 26]; 26 * 26]);
impl AdjMatrix {
    fn new() -> Self {
        AdjMatrix([[false; 26 * 26]; 26 * 26])
    }

    fn are_neigh(&self, pc1: &PC, pc2: &PC) -> bool {
        self.0[pc_to_cord(pc1)][pc_to_cord(pc2)]
    }

    fn set_neigh(&mut self, pc1: &PC, pc2: &PC) {
        self.0[pc_to_cord(pc1)][pc_to_cord(pc2)] = true;
        self.0[pc_to_cord(pc2)][pc_to_cord(pc1)] = true;
    }

    fn are_neigh_cord(&self, cord1: usize, cord2: usize) -> bool {
        self.0[cord1][cord2]
    }
}

#[inline]
fn pc_to_cord(pc: &PC) -> usize {
    (pc.0 as u16 * 26 + pc.1 as u16) as usize
}

fn pc_to_str(pc: &PC) -> String {
    ((pc.0 + b'a') as char).to_string() + &((pc.1 + b'a') as char).to_string()
}

fn cord_to_pc(cord: usize) -> PC {
    ((cord / 26) as u8, (cord % 26) as u8)
}

#[inline]
fn char_to_num(c: char) -> u8 {
    c as u8 - b'a'
}

#[inline]
fn starts_with_t(pc: &PC) -> bool {
    pc.0 == T
}

fn parse_input(input: &str) -> (AdjMatrix, Vec<(u8, u8)>) {
    let mut adj: AdjMatrix = AdjMatrix::new();
    let mut pcs: HashSet<(u8, u8)> = HashSet::new();
    input.lines().for_each(|l| {
        let mut it = l.chars();
        let a = (
            char_to_num(it.next().unwrap()),
            char_to_num(it.next().unwrap()),
        );
        it.next();
        let b = (
            char_to_num(it.next().unwrap()),
            char_to_num(it.next().unwrap()),
        );
        adj.set_neigh(&a, &b);
        pcs.insert(a);
        pcs.insert(b);
    });
    assert!(pcs.iter().all(|pc| pc.0 <= 25 && pc.1 <= 25));
    (adj, pcs.into_iter().collect::<Vec<(u8, u8)>>())
}

pub fn process_part1(input: &str) -> u64 {
    let (adj, pcs) = parse_input(input);
    pcs.iter()
        .tuple_combinations::<(&PC, &PC, &PC)>()
        .filter(|(pc1, pc2, pc3)| {
            (starts_with_t(pc1) || starts_with_t(pc2) || starts_with_t(pc3))
                && adj.are_neigh(pc1, pc2)
                && adj.are_neigh(pc1, pc3)
                && adj.are_neigh(pc2, pc3)
        })
        .count() as u64
}

pub fn process_part2(input: &str) -> String {
    let mut cliques: HashSet<Vec<usize>> = HashSet::new();
    let (adj, pcs) = parse_input(input);
    pcs.into_iter()
        .for_each(|pc| expand_cliques(&mut cliques, &adj, pc_to_cord(&pc)));
    let mut max_clique = cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .clone();
    max_clique.sort();
    max_clique
        .into_iter()
        .map(|cord| pc_to_str(&cord_to_pc(cord)))
        .join(",")
}

fn expand_cliques(cliques: &mut HashSet<Vec<usize>>, adj: &AdjMatrix, v: usize) {
    let mut new_cliques: Vec<Vec<usize>> = cliques
        .iter()
        .filter(|clique| clique.iter().all(|pc| adj.are_neigh_cord(*pc, v)))
        .map(|clique| {
            let mut c = clique.clone();
            c.push(v);
            c
        })
        .collect();
    new_cliques.push(vec![v]);
    cliques.extend(new_cliques);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn pc_to_cord_bijection() {
        let mut arr = [false; 26 * 26];
        (0..26_u8)
            .cartesian_product(0..26_u8)
            .map(|pc| pc_to_cord(&pc))
            .inspect(|a| println!("{}", a))
            .for_each(|cord| arr[cord] = true);
        println!("{:?}", arr);
        assert!(arr.iter().all(|b| *b));
    }

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), 7);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE), "co,de,ka,ta");
    }
}
