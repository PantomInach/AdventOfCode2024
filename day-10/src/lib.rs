use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    height: u8,
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) ^{}", self.x, self.y, self.height)
    }
}

struct Graph {
    nodes: Vec<Node>,
    adj_map: HashMap<Node, Vec<Node>>,
}
impl From<&str> for Graph {
    fn from(input: &str) -> Self {
        let mut adj_map: HashMap<Node, Vec<Node>> = HashMap::new();
        let grid: Vec<Vec<Node>> = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| Node {
                        x,
                        y,
                        height: c as u8 - b'0',
                    })
                    .collect::<Vec<Node>>()
            })
            .collect();
        let y_bound = grid.len();
        let x_bound = grid.first().unwrap().len();

        (0..y_bound).for_each(|y| {
            (0..x_bound).for_each(|x| {
                let center = grid.get(y).and_then(|l| l.get(x)).unwrap();
                if let Some(node) = x
                    .checked_sub(1)
                    .and_then(|x| grid.get(y).and_then(|l| l.get(x)))
                    .and_then(|node| (node.height == center.height + 1).then_some(node))
                {
                    adj_map
                        .entry(*center)
                        .and_modify(|adjs| adjs.push(*node))
                        .or_insert(vec![*node]);
                }
                if let Some(node) = y
                    .checked_sub(1)
                    .and_then(|y| grid.get(y).and_then(|l| l.get(x)))
                    .and_then(|node| (node.height == center.height + 1).then_some(node))
                {
                    adj_map
                        .entry(*center)
                        .and_modify(|adjs| adjs.push(*node))
                        .or_insert(vec![*node]);
                }
                if let Some(node) = (x + 1 < x_bound)
                    .then_some(grid.get(y).and_then(|l| l.get(x + 1)))
                    .flatten()
                    .and_then(|node| (node.height == center.height + 1).then_some(node))
                {
                    adj_map
                        .entry(*center)
                        .and_modify(|adjs| adjs.push(*node))
                        .or_insert(vec![*node]);
                }
                if let Some(node) = (y + 1 < y_bound)
                    .then_some(grid.get(y + 1).and_then(|l| l.get(x)))
                    .flatten()
                    .and_then(|node| (node.height == center.height + 1).then_some(node))
                {
                    adj_map
                        .entry(*center)
                        .and_modify(|adjs| adjs.push(*node))
                        .or_insert(vec![*node]);
                }
            })
        });

        Graph {
            nodes: grid.iter().flatten().copied().collect::<Vec<Node>>(),
            adj_map,
        }
    }
}
impl Graph {
    fn trailhead_positions(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(pos, n)| (n.height == 0).then_some(pos))
            .collect()
    }

    fn count_unique_trailfoots(&self, trailhead_pos: usize) -> u64 {
        let mut stack: Vec<Node> = vec![*self.nodes.get(trailhead_pos).unwrap()];
        let mut trailfoots: HashSet<Node> = HashSet::new();
        while let Some(node) = stack.pop() {
            if node.height == 9 {
                trailfoots.insert(node);
            } else if let Some(nodes) = self.adj_map.get(&node) {
                stack.extend(nodes);
            }
        }
        trailfoots.len() as u64
    }

    fn count_trails(&self, trailhead_pos: usize) -> u64 {
        let mut stack: Vec<Node> = vec![*self.nodes.get(trailhead_pos).unwrap()];
        let mut trailfoots: Vec<Node> = Vec::new();
        while let Some(node) = stack.pop() {
            if node.height == 9 {
                trailfoots.push(node);
            } else if let Some(nodes) = self.adj_map.get(&node) {
                stack.extend(nodes);
            }
        }
        trailfoots.len() as u64
    }
}

pub fn process_part1(input: &str) -> u64 {
    let graph = Graph::from(input);
    graph
        .trailhead_positions()
        .iter()
        .map(|pos| graph.count_unique_trailfoots(*pos))
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    let graph = Graph::from(input);
    graph
        .trailhead_positions()
        .iter()
        .map(|pos| graph.count_trails(*pos))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), 36);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE), 81)
    }
}
