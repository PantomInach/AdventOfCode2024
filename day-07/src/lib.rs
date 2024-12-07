fn parse_input(s: &str) -> Vec<(u64, Vec<u64>)> {
    s.lines()
        .map(|l| {
            let (n, nums) = l.split_once(':').unwrap();
            (
                n.parse::<u64>().unwrap(),
                nums.split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

pub fn process_part1(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .filter(|(total, nums)| equation_possible(*total, nums))
        .map(|(total, _)| total)
        .sum()
}

fn equation_possible(total: u64, nums: &[u64]) -> bool {
    rec(total, &mut nums.to_vec())
}

fn rec(total: u64, nums: &mut Vec<u64>) -> bool {
    if nums.is_empty() {
        return total == 0;
    }
    if total == 0 {
        return false;
    }
    let n = nums.pop().unwrap();
    let res = if total < n {
        false
    } else if total % n == 0 && rec(total / n, nums) {
        true
    } else {
        rec(total - n, nums)
    };
    nums.push(n);
    res
}

pub fn process_part2(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .filter(|(total, nums)| {
            let possible = equation_possible2(*total, nums);
            if !possible {
                println!("{}: {:?}", total, nums);
            }
            possible
        })
        .map(|(total, _)| total)
        .sum()
}

fn equation_possible2(total: u64, nums: &[u64]) -> bool {
    rec2(total, &mut nums.to_owned())
}

fn rec2(total: u64, nums: &mut Vec<u64>) -> bool {
    if nums.is_empty() {
        return total == 0;
    }
    if total == 0 {
        return false;
    }
    let n = nums.pop().unwrap();
    let res = if total < n {
        false
    } else if total % n == 0 && rec2(total / n, nums)
        || total % u64::pow(10, n.ilog10() + 1) == n
            && rec2(total / (u64::pow(10, n.ilog10() + 1)), nums)
    {
        true
    } else {
        rec2(total - n, nums)
    };
    nums.push(n);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), 3749);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE), 11387)
    }

    #[test]
    fn test_rec2() {
        assert!(rec2(12011121824, &mut vec![15, 8, 11, 12, 1, 824]));
    }
}
