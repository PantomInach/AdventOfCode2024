use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
enum MapItem {
    Obstical,
    Walked(Dir),
    Unwalked,
}
impl Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MapItem::Obstical => "#".to_string(),
                MapItem::Walked(dir) => {
                    dir.to_string().clone()
                }
                MapItem::Unwalked => ".".to_string(),
            }
        )
    }
}
#[derive(PartialEq, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Up => '^',
                Dir::Right => '>',
                Dir::Down => 'V',
                Dir::Left => '<',
            }
        )
    }
}
impl Dir {
    fn rotate(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Clone, Copy)]
struct Guard {
    x: usize,
    y: usize,
    dir: Dir,
}
impl Guard {
    fn rotate(&mut self) {
        self.dir = self.dir.rotate();
    }

    fn set_cords(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn next_step_pos(&self) -> Option<(usize, usize)> {
        match self.dir {
            Dir::Up => self.y.checked_sub(1).map(|y| (self.x, y)),
            Dir::Right => Some((self.x + 1, self.y)),
            Dir::Down => Some((self.x, self.y + 1)),
            Dir::Left => self.x.checked_sub(1).map(|x| (x, self.y)),
        }
    }

    fn cycles(&self, map: &Vec<Vec<MapItem>>) -> bool {
        if let Some(MapItem::Walked(dir)) = self
            .next_step_pos()
            .and_then(|(x, y)| map.get(y).and_then(|l| l.get(x)))
        {
            *dir == self.dir
        } else {
            false
        }
    }

    fn action(&mut self, map: &mut Vec<Vec<MapItem>>) -> bool {
        if let Some((x, y)) = self.next_step_pos() {
            if let Some(item) = map.get_mut(y).and_then(|l| l.get_mut(x)) {
                match item {
                    MapItem::Obstical => self.rotate(),
                    MapItem::Unwalked => {
                        *item = MapItem::Walked(self.dir);
                        self.set_cords(x, y);
                    }
                    MapItem::Walked(_) => self.set_cords(x, y),
                };
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn parse_input(input: &str) -> (Guard, Vec<Vec<MapItem>>) {
    let mut guard = Guard {
        x: 0,
        y: 0,
        dir: Dir::Up,
    };
    let map = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => MapItem::Obstical,
                    '.' => MapItem::Unwalked,
                    '^' => {
                        guard = Guard {
                            x: j,
                            y: i,
                            dir: Dir::Up,
                        };
                        MapItem::Walked(Dir::Up)
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (guard, map)
}

pub fn process_part1(input: &str) -> u64 {
    let mut parsed = parse_input(input);
    let guard = &mut parsed.0;
    let map = &mut parsed.1;
    while guard.action(map) {}

    map.iter().for_each(|l| {
        println!(
            "{}",
            l.iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    });
    map.iter()
        .map(|l| l.iter().filter(|t| matches!(t, MapItem::Walked(_))).count())
        .sum::<usize>() as u64
}

pub fn process_part2(input: &str) -> u64 {
    let (original_guard, original_map) = parse_input(input);
    let guard = &mut original_guard.clone();
    let map = &mut original_map.clone();
    while guard.action(map) {}

    let possible_obstical_positions: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, t)| matches!(t, MapItem::Walked(_)))
                .map(move |(x, _)| (x, y))
        })
        .collect();

    possible_obstical_positions
        .iter()
        .filter(|(x, y)| {
            let guard: &mut Guard = &mut original_guard.clone();
            let map: &mut Vec<Vec<MapItem>> = &mut original_map.clone();
            if let Some(tile) = map.get_mut(*y).and_then(|l| l.get_mut(*x)) {
                *tile = MapItem::Obstical;
            } else {
                return false;
            };

            while guard.action(map) {
                if guard.cycles(map) {
                    return true;
                }
            }
            false
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), 41);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE), 6)
    }
}
