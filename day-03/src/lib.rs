use regex::Regex;

enum Func {
    Mul(u64, u64),
    Do,
    Dont,
}

fn parse_mul(s: &str) -> (u64, u64) {
    let (x, y) = s
        .split_once("(")
        .unwrap()
        .1
        .trim_end_matches(")")
        .split_once(",")
        .unwrap();
    (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
}

fn mul(s: &str) -> u64 {
    let (x, y) = parse_mul(s);
    x * y
}

fn parse_func(s: &str) -> Func {
    match s {
        "don't()" => Func::Dont,
        "do()" => Func::Do,
        _ => {
            let (x, y) = parse_mul(s);
            Func::Mul(x, y)
        }
    }
}

pub fn process_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            Regex::new(r"mul\(\d{1,3},\d{1,3}\)")
                .unwrap()
                .find_iter(l)
                .map(|m| mul(m.as_str()))
                .sum::<u64>()
        })
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    let mut total: u64 = 0;
    let mut enabled: u64 = 1;
    input
        .lines()
        .flat_map(|l| {
            Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)")
                .unwrap()
                .find_iter(l)
                .map(|m| parse_func(m.as_str()))
                .collect::<Vec<Func>>()
        })
        .for_each(|f| match f {
            Func::Mul(x, y) => total += x * y * enabled,
            Func::Do => enabled = 1,
            Func::Dont => enabled = 0,
        });
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process_part1(input), 161);
    }

    #[test]
    fn test_process_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(process_part2(input), 48)
    }
}
