use std::collections::HashMap;

use itertools::Itertools;

#[allow(clippy::type_complexity)]
fn parse_input(s: &str) -> (i64, i64, HashMap<char, Vec<(i64, i64)>>) {
    let y_bounds: i64 = s.lines().count() as i64 - 1;
    let x_bounds: i64 = s.lines().next().unwrap().len() as i64 - 1;
    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    s.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                antennas.entry(c).or_default().push((x as i64, y as i64));
            }
        });
    });
    (x_bounds, y_bounds, antennas)
}

#[inline]
fn is_in_bound(x: i64, y: i64, x_bound: i64, y_bound: i64) -> bool {
    0 <= x && x <= x_bound && 0 <= y && y <= y_bound
}

fn anti_antennas(x1: i64, y1: i64, x2: i64, y2: i64) -> Vec<(i64, i64)> {
    let x_diff = x1 - x2;
    let y_diff = y1 - y2;
    vec![(x1 + x_diff, y1 + y_diff), (x2 - x_diff, y2 - y_diff)]
}

pub fn process_part1(input: &str) -> u64 {
    let (x_bound, y_bound, antennas) = parse_input(input);
    // println!("Antennas: {:?}", antennas);
    antennas
        .iter()
        .flat_map(|(_, positions)| {
            positions
                .iter()
                .tuple_combinations()
                .flat_map(|((x1, y1), (x2, y2))| anti_antennas(*x1, *y1, *x2, *y2))
                .filter(|(x, y)| is_in_bound(*x, *y, x_bound, y_bound))
        })
        .unique()
        .count() as u64
}

fn harmonic_anti_antennas(
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    x_bound: i64,
    y_bound: i64,
) -> Vec<(i64, i64)> {
    let x_diff = x1 - x2;
    let y_diff = y1 - y2;
    let mut anti_antennas_pos: Vec<(i64, i64)> = vec![];
    let mut cords: (i64, i64) = (x1, y1);
    while is_in_bound(cords.0, cords.1, x_bound, y_bound) {
        anti_antennas_pos.push(cords);
        cords.0 += x_diff;
        cords.1 += y_diff;
    }
    cords = (x2, y2);
    while is_in_bound(cords.0, cords.1, x_bound, y_bound) {
        anti_antennas_pos.push(cords);
        cords.0 -= x_diff;
        cords.1 -= y_diff;
    }
    anti_antennas_pos
}

pub fn process_part2(input: &str) -> u64 {
    let (x_bound, y_bound, antennas) = parse_input(input);
    // println!("Antennas: {:?}", antennas);
    antennas
        .iter()
        .flat_map(|(_, positions)| {
            positions
                .iter()
                .tuple_combinations()
                .flat_map(|((x1, y1), (x2, y2))| {
                    harmonic_anti_antennas(*x1, *y1, *x2, *y2, x_bound, y_bound)
                })
        })
        .unique()
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), 14);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE), 34)
    }
}
