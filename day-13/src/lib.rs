fn line_numbers(input: &str) -> [i64; 2] {
    let mut ns = [0, 0];
    let mut num_prev = false;
    let mut cur = 0;
    input.chars().for_each(|c| match c {
        '0'..='9' => {
            num_prev = true;
            ns[cur] = 10 * ns[cur] + (c as u8 - b'0') as i64;
        }
        _ if num_prev => {
            num_prev = false;
            cur += 1;
        }
        _ => (),
    });
    ns
}

fn parse_input(input: &str) -> Vec<(i64, i64, i64, i64, i64, i64)> {
    let mut lines = input.lines().peekable();
    let mut res = vec![];
    while lines.peek().is_some() {
        let l1 = line_numbers(lines.next().unwrap());
        let l2 = line_numbers(lines.next().unwrap());
        let l3 = line_numbers(lines.next().unwrap());
        lines.next();
        res.push((l1[0], l1[1], l2[0], l2[1], l3[0], l3[1]));
    }
    res
}

pub fn process_part1(input: &str) -> i64 {
    parse_input(input)
        .iter()
        .filter_map(|(a1, b1, a2, b2, x, y)| {
            let b_presses_num = y * a1 - x * b1;
            let b_presses_dom = b2 * a1 - a2 * b1;
            if b_presses_num % b_presses_dom != 0 {
                return None;
            }
            let b_presses = b_presses_num / b_presses_dom;
            let a_presses_num = x - b_presses * a2;
            if a_presses_num % a1 != 0 {
                return None;
            }
            let a_presses = a_presses_num / a1;
            Some(3 * a_presses + b_presses)
        })
        .sum()
}

pub fn process_part2(input: &str) -> i64 {
    parse_input(input)
        .iter()
        .map(|(a1, b1, a2, b2, x, y)| (a1, b1, a2, b2, x + 10000000000000, y + 10000000000000))
        .filter_map(|(a1, b1, a2, b2, x, y)| {
            let b_presses_num = y * a1 - x * b1;
            let b_presses_dom = b2 * a1 - a2 * b1;
            if b_presses_num % b_presses_dom != 0 {
                return None;
            }
            let b_presses = b_presses_num / b_presses_dom;
            let a_presses_num = x - b_presses * a2;
            if a_presses_num % a1 != 0 {
                return None;
            }
            let a_presses = a_presses_num / a1;
            Some(3 * a_presses + b_presses)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(EXAMPLE),
            vec![
                (94, 34, 22, 67, 8400, 5400),
                (26, 66, 67, 21, 12748, 12176),
                (17, 86, 84, 37, 7870, 6450),
                (69, 23, 27, 71, 18641, 10279)
            ]
        );
    }

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), 480);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE), 55312);
    }
}
