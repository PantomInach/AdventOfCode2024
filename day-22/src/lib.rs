use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<Secret> {
    input
        .lines()
        .map(|l| Secret(l.parse::<u64>().unwrap()))
        .collect()
}

struct Secret(u64);
impl Iterator for Secret {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.0;
        self.mix(self.0 * 64).prune();
        self.mix(self.0 / 32).prune();
        self.mix(self.0 * 2048).prune();
        Some(ret)
    }
}
impl Secret {
    #[inline]
    fn mix(&mut self, other: u64) -> &mut Secret {
        self.0 ^= other;
        self
    }

    #[inline]
    fn prune(&mut self) -> &mut Secret {
        self.0 %= 16777216;
        self
    }
}

pub fn process_part1(input: &str) -> u64 {
    parse_input(input)
        .iter_mut()
        .map(|sec| sec.nth(2000).unwrap())
        .sum()
}

fn score(sec: &mut Secret, to: usize, scoring: &mut HashMap<[i8; 4], u64>) {
    let mut seen: HashSet<[i8; 4]> = HashSet::new();
    let n_bannanas: Vec<i8> = sec.take(to).map(|n| (n % 10) as i8).collect();
    let diffs: Vec<i8> = n_bannanas
        .iter()
        .as_slice()
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect();
    diffs
        .as_slice()
        .windows(4)
        .zip(n_bannanas.into_iter().skip(4))
        .for_each(|(arr, n)| {
            if !seen.contains(arr) {
                scoring
                    .entry([arr[0], arr[1], arr[2], arr[3]])
                    .and_modify(|e| *e += n as u64)
                    .or_insert(n as u64);
                seen.insert([arr[0], arr[1], arr[2], arr[3]]);
            }
        });
}

pub fn process_part2(input: &str) -> u64 {
    let mut scoring: HashMap<[i8; 4], u64> = HashMap::new();
    parse_input(input)
        .iter_mut()
        .for_each(|sec| score(sec, 2000, &mut scoring));
    scoring
        .iter()
        .max_by_key(|(_, v)| *v)
        .map(|(_, v)| *v)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1
10
100
2024";

    const EXAMPLE2: &str = "1
2
3
2024";

    #[test]
    fn test_secret() {
        let mut sec = Secret(123).into_iter();
        sec.next();
        assert_eq!(sec.next(), Some(15887950));
        assert_eq!(sec.next(), Some(16495136));
        assert_eq!(sec.next(), Some(527345));
        assert_eq!(sec.next(), Some(704524));
        assert_eq!(sec.next(), Some(1553684));
        assert_eq!(sec.next(), Some(12683156));
        assert_eq!(sec.next(), Some(11100544));
        assert_eq!(sec.next(), Some(12249484));
        assert_eq!(sec.next(), Some(7753432));
        assert_eq!(sec.next(), Some(5908254));
    }

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), 37327623);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE2), 23);
    }
}
