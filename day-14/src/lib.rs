use std::{collections::HashMap, vec};

const MAX_X: i64 = 101;
const MAX_Y: i64 = 103;

#[derive(Clone, Copy)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}
impl Robot {
    fn move_bot(&mut self, x_bound: i64, y_bound: i64) {
        self.x = (self.x + x_bound + self.vx) % x_bound;
        self.y = (self.y + y_bound + self.vy) % y_bound;
    }

    #[allow(clippy::comparison_chain)]
    fn quadrant(&self, x_bound: i64, y_bound: i64) -> Option<u8> {
        let mut quad = 0;
        if self.x == (x_bound - 1) / 2 {
            return None;
        } else if self.x > (x_bound - 1) / 2 {
            quad += 1;
        }
        if self.y == (y_bound - 1) / 2 {
            return None;
        } else if self.y > (y_bound - 1) / 2 {
            quad += 2;
        }
        Some(quad + 1)
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (x, y) = p.split_once("=").unwrap().1.split_once(",").unwrap();
            let (vx, vy) = v.split_once("=").unwrap().1.split_once(",").unwrap();
            Robot {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect()
}

#[inline]
fn move_robots(x_bound: i64, y_bound: i64, robots: &mut [Robot]) {
    robots.iter_mut().for_each(|r| r.move_bot(x_bound, y_bound));
}

fn process_robots(sec: usize, x_bound: i64, y_bound: i64, robots: &mut [Robot]) -> u64 {
    (0..sec).for_each(|_| move_robots(x_bound, y_bound, robots));
    let mut counter: HashMap<u8, u64> = HashMap::new();
    robots
        .iter()
        .filter_map(|r| r.quadrant(x_bound, y_bound))
        .for_each(|q| {
            counter
                .entry(q)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });
    counter.values().fold(1_u64, |acc, q| acc * (*q))
}

pub fn process_part1(input: &str) -> u64 {
    process_robots(100, MAX_X, MAX_Y, &mut parse_input(input))
}

fn robots_to_field(x_bound: i64, y_bound: i64, robots: &[Robot]) -> Vec<Vec<bool>> {
    let mut field: Vec<Vec<bool>> = vec![vec![false; x_bound as usize]; y_bound as usize];
    robots.iter().for_each(|r| {
        if let Some(b) = field
            .get_mut(r.y as usize)
            .and_then(|l| l.get_mut(r.x as usize))
        {
            *b = true;
        }
    });
    field
}

fn robots_visualize(x_bound: i64, y_bound: i64, robots: &[Robot]) {
    let field = robots_to_field(x_bound, y_bound, robots);
    println!(
        "{}",
        field
            .iter()
            .map(|l| l
                .iter()
                .map(|b| if *b { "#" } else { " " }.to_string())
                .collect::<String>()
                + "\n")
            .collect::<String>()
    );
}

fn field_has_horizontal_line(size: usize, field: Vec<Vec<bool>>) -> bool {
    field.iter().any(|line| {
        line.as_slice()
            .windows(size)
            .any(|bs| bs.iter().all(|b| *b))
    })
}

/// We know that the Christmas tree has a frame consisting of four straight lines.
/// We use this fact to search for an large enough horizontal line.
pub fn process_part2(input: &str) -> u64 {
    let mut robots = parse_input(input);
    let mut field = robots_to_field(MAX_X, MAX_Y, &robots);
    let mut i = 0;
    while !field_has_horizontal_line(13, field) {
        move_robots(MAX_X, MAX_Y, &mut robots);
        field = robots_to_field(MAX_X, MAX_Y, &robots);
        i += 1;
    }
    robots_visualize(MAX_X, MAX_Y, &robots);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_robots(100, 11, 7, &mut parse_input(EXAMPLE)), 12);
    }

    #[test]
    fn test_process_part2() {}
}
