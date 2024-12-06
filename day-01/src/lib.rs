use std::collections::HashMap;

pub fn process_part1(input: &str) -> u64 {
    let mut l1: Vec<u64> = vec![];
    let mut l2: Vec<u64> = vec![];
    input.lines().for_each(|l| {
        let mut nums = l.split_whitespace().map(|n| n.parse::<u64>().unwrap());
        l1.push(nums.next().unwrap());
        l2.push(nums.next().unwrap());
    });
    l1.sort();
    l2.sort();
    l1.into_iter()
        .zip(l2)
        .map(|(x, y)| u64::abs_diff(x, y))
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    let mut l1: Vec<u64> = vec![];
    let mut m2: HashMap<u64, u64> = HashMap::new();
    input.lines().for_each(|l| {
        let mut nums = l.split_whitespace().map(|n| n.parse::<u64>().unwrap());
        l1.push(nums.next().unwrap());
        m2.entry(nums.next().unwrap())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    });
    l1.iter()
        .map(|n| m2.get(n).map(|v| n * v).unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(process_part1(input), 11);
    }

    #[test]
    fn test_process_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(process_part2(input), 31)
    }
}
