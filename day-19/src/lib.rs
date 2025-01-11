use std::collections::HashMap;

type Desing = Vec<char>;

fn parse_input(input: &str) -> (Vec<Desing>, Vec<Desing>) {
    let mut lines = input.lines();
    let patterns: Vec<Desing> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|p| p.split_once(",").unwrap_or((p, "")).0.chars().collect())
        .collect();
    lines.next();
    let designs: Vec<Desing> = lines.map(|l| l.chars().collect()).collect();
    (designs, patterns)
}

fn ends_with(pattern: &Desing, design: &Desing) -> bool {
    if pattern.len() > design.len() {
        return false;
    }
    pattern
        .iter()
        .rev()
        .zip(design.iter().rev().take(pattern.len()))
        .all(|(x, y)| x == y)
}

fn is_creatable(design: &Desing, patterns: &Vec<Desing>) -> bool {
    rec(&mut design.clone(), patterns)
}

fn rec(design: &mut Desing, patterns: &Vec<Desing>) -> bool {
    if design.is_empty() {
        return true;
    }
    patterns.iter().any(|pat| {
        if !ends_with(pat, design) {
            return false;
        }
        let store = design.split_off(design.len() - pat.len());
        let res = rec(design, patterns);
        design.extend(store);
        res
    })
}

fn count_creatable(design: &Desing, patterns: &Vec<Desing>) -> usize {
    rec_count(&mut design.clone(), patterns, &mut HashMap::new())
}

fn rec_count(
    design: &mut Desing,
    patterns: &Vec<Desing>,
    cache: &mut HashMap<Desing, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(c) = cache.get(design) {
        return *c;
    }
    let c = patterns
        .iter()
        .filter_map(|pat| {
            if !ends_with(pat, design) {
                return None;
            }
            let store = design.split_off(design.len() - pat.len());
            let res = rec_count(design, patterns, cache);
            design.extend(store);
            Some(res)
        })
        .sum();
    cache.insert(design.to_vec(), c);
    c
}

pub fn process_part1(input: &str) -> u64 {
    let (designs, patterns) = parse_input(input);
    designs
        .iter()
        .filter(|d| is_creatable(d, &patterns))
        .count() as u64
}

pub fn process_part2(input: &str) -> u64 {
    let (designs, patterns) = parse_input(input);
    designs
        .iter()
        .map(|d| count_creatable(d, &patterns))
        .sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br
    
brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), 6);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE), 16);
    }
}
