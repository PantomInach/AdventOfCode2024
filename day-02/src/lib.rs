const MAX_DIFF: u64 = 3;
const MIN_DIFF: u64 = 1;

fn is_inc(nums: &Vec<u64>) -> bool {
    nums.as_slice().windows(2).all(|a| inc(a[0], a[1]))
}

fn is_dec(nums: &Vec<u64>) -> bool {
    nums.as_slice().windows(2).all(|a| dec(a[0], a[1]))
}

pub fn process_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|w| w.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .filter(|nums| is_inc(nums) || is_dec(nums))
        .count() as u64
}

#[inline]
fn inc(x: u64, y: u64) -> bool {
    MAX_DIFF >= u64::abs_diff(x, y) && u64::abs_diff(x, y) >= MIN_DIFF && x < y
}

#[inline]
fn dec(x: u64, y: u64) -> bool {
    MAX_DIFF >= u64::abs_diff(x, y) && u64::abs_diff(x, y) >= MIN_DIFF && x > y
}

fn is_inc_enough(nums: &Vec<u64>) -> bool {
    nums.as_slice()
        .windows(2)
        .position(|a| !inc(a[0], a[1]))
        .is_none_or(|p| {
            let mut ns1 = nums.clone();
            ns1.remove(p);
            let mut ns2 = nums.clone();
            ns2.remove(p + 1);
            is_inc(&ns1) || is_inc(&ns2)
        })
}

fn is_dec_enough(nums: &Vec<u64>) -> bool {
    nums.as_slice()
        .windows(2)
        .position(|a| !dec(a[0], a[1]))
        .is_none_or(|p| {
            let mut ns1 = nums.clone();
            ns1.remove(p);
            let mut ns2 = nums.clone();
            ns2.remove(p + 1);
            is_dec(&ns1) || is_dec(&ns2)
        })
}

pub fn process_part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|w| w.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .filter(|nums| is_inc_enough(nums) || is_dec_enough(nums))
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(process_part1(input), 2);
    }

    #[test]
    fn test_process_part2() {
        let input: Vec<u64> = vec![1, 2, 4, 7, 9, 8];
        assert!(is_inc_enough(&input));
        let input: Vec<u64> = vec![43, 44, 47, 49, 49];
        assert!(is_inc_enough(&input));
        let input: Vec<u64> = vec![6, 7, 9, 11, 13, 14, 18];
        assert!(is_inc_enough(&input));
        let input: Vec<u64> = vec![34, 35, 38, 39, 42, 48];
        assert!(is_inc_enough(&input));
        let input: Vec<u64> = vec![73, 76, 79, 80, 82, 85, 83, 86];
        assert!(is_inc_enough(&input));
        let input: Vec<u64> = vec![64, 67, 69, 66, 69, 68];
        assert!(!is_inc_enough(&input));
        let input: Vec<u64> = vec![43, 45, 48, 50, 49, 49];
        assert!(!is_inc_enough(&input));
        let input: Vec<u64> = vec![77, 78, 81, 78, 81, 83, 87];
        assert!(!is_inc_enough(&input));
        let input: Vec<u64> = vec![3, 4, 7, 8, 6, 12];
        assert!(!is_inc_enough(&input));
        let input: Vec<u64> = vec![73, 75, 75, 76, 77, 80];
        assert!(is_inc_enough(&input));
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(process_part2(input), 4)
    }
}
