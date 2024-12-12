use std::collections::HashMap;

type AccStones = HashMap<u64, usize>;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn blink_one_stone(stone: &u64) -> Vec<u64> {
    match stone {
        0 => vec![1],
        _ if stone.ilog10() % 2 == 1 => vec![
            stone / u64::pow(10, (stone.ilog10() + 1) / 2),
            stone % u64::pow(10, (stone.ilog10() + 1) / 2),
        ],
        _ => vec![stone * 2024],
    }
}

fn blink(stones: &[u64]) -> Vec<u64> {
    stones.iter().flat_map(blink_one_stone).collect()
}

pub fn process_part1(input: &str) -> u64 {
    (0..25).fold(parse_input(input), |acc, _| blink(&acc)).len() as u64
}

fn accumulate(stones: &[(u64, usize)]) -> AccStones {
    let mut count: HashMap<u64, usize> = HashMap::new();
    stones.iter().for_each(|(stone, n)| {
        count
            .entry(*stone)
            .and_modify(|present| *present += n)
            .or_insert(*n);
    });
    count
}

fn blink_acc(stones: &AccStones) -> Vec<(u64, usize)> {
    stones
        .iter()
        .flat_map(|(stone, n)| {
            blink_one_stone(stone)
                .iter()
                .map(|s| (*s, *n))
                .collect::<Vec<(u64, usize)>>()
        })
        .collect()
}

fn better_blink_n_times(stones: AccStones, n: usize) -> u64 {
    (0..n)
        .fold(stones, |acc, _| accumulate(&blink_acc(&acc)))
        .values()
        .sum::<usize>() as u64
}

pub fn process_part2(input: &str) -> u64 {
    better_blink_n_times(
        accumulate(
            &parse_input(input)
                .iter()
                .map(|stone| (*stone, 1))
                .collect::<Vec<(u64, usize)>>(),
        ),
        75,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn test_process_part1() {
        assert_eq!(blink_one_stone(&1000), vec![10, 0]);
        assert_eq!(blink_one_stone(&1010), vec![10, 10]);
        assert_eq!(blink_one_stone(&101), vec![101 * 2024]);
        assert_eq!(blink_one_stone(&253000), vec![253, 0]);
        assert_eq!(process_part1(EXAMPLE), 55312);
    }

    #[test]
    fn test_process_part2() {
        let stones = accumulate(
            &parse_input(EXAMPLE)
                .iter()
                .map(|stone| (*stone, 1))
                .collect::<Vec<(u64, usize)>>(),
        );
        assert_eq!(better_blink_n_times(stones, 25), 55312);
    }
}
