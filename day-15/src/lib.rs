use std::collections::HashMap;

enum Dir {
    Up,
    Right,
    Down,
    Left,
}
impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '^' => Dir::Up,
            '>' => Dir::Right,
            'v' => Dir::Down,
            '<' => Dir::Left,
            _ => unreachable!(),
        }
    }
}
impl Dir {
    fn apply(&self, x: i64, y: i64) -> (i64, i64) {
        match self {
            Dir::Up => (x, y - 1),
            Dir::Right => (x + 1, y),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Tile {
    Box,
    Wall,
}

struct Robot {
    x: i64,
    y: i64,
}
impl Robot {
    fn new_cords(&self, dir: &Dir) -> (i64, i64) {
        dir.apply(self.x, self.y)
    }

    fn move_dir(&mut self, dir: &Dir) {
        let (x, y) = dir.apply(self.x, self.y);
        self.x = x;
        self.y = y;
    }
}

fn parse_input(input: &str) -> (HashMap<(i64, i64), Tile>, Vec<Dir>, Robot) {
    let mut tiles: HashMap<(i64, i64), Tile> = HashMap::new();
    let mut lines = input.lines();
    let mut robot = Robot { x: 0, y: 0 };
    let mut y = 0;
    while let Some(Some(l)) = lines.next().map(|l| l.contains('#').then(|| l)) {
        let mut x = 0;
        l.chars().for_each(|c| {
            match c {
                '#' => {
                    tiles.insert((x, y), Tile::Wall);
                }
                'O' => {
                    tiles.insert((x, y), Tile::Box);
                }
                '.' => {}
                '@' => {
                    robot.x = x;
                    robot.y = y;
                }
                _ => unreachable!(),
            };
            x += 1;
        });
        y += 1;
    }
    let moves = lines
        .flat_map(|l| l.chars().map(|c| Dir::from(c)))
        .collect::<Vec<Dir>>();
    (tiles, moves, robot)
}

fn boxes_end_at(
    xx: i64,
    yy: i64,
    tiles: &HashMap<(i64, i64), Tile>,
    dir: &Dir,
) -> Option<(i64, i64)> {
    let (mut x, mut y) = dir.apply(xx, yy);
    while let Some(t) = tiles.get(&(x, y)) {
        match t {
            Tile::Box => {
                (x, y) = dir.apply(x, y);
            }
            Tile::Wall => {
                return None;
            }
        }
    }
    Some((x, y))
}

fn print_tiles(tiles: &HashMap<(i64, i64), Tile>) {
    let max_x = tiles.iter().max_by_key(|((x, _), _)| x).unwrap().0 .0 + 1;
    let max_y = tiles.iter().max_by_key(|((_, y), _)| y).unwrap().0 .1 + 1;
    let mut field = vec![vec![".".to_string(); max_x as usize]; max_y as usize];
    tiles.iter().for_each(|((x, y), t)| {
        *field
            .get_mut(*y as usize)
            .and_then(|l| l.get_mut(*x as usize))
            .unwrap() = if *t == Tile::Wall {
            "#".to_string()
        } else {
            "O".to_string()
        }
    });
    println!(
        "{}",
        field
            .into_iter()
            .map(|l| l.into_iter().collect::<String>() + "\n")
            .collect::<String>()
    );
}

pub fn process_part1(input: &str) -> u64 {
    let (mut tiles, moves, mut robot) = parse_input(input);
    moves.iter().for_each(|dir| {
        match tiles.get(&robot.new_cords(dir)) {
            Some(Tile::Wall) => (),
            Some(Tile::Box) => {
                if let Some((x, y)) = boxes_end_at(robot.x, robot.y, &tiles, dir) {
                    tiles.insert((x, y), Tile::Box);
                    tiles.remove(&robot.new_cords(dir));
                    robot.move_dir(dir);
                }
            }
            None => {
                robot.move_dir(dir);
            }
        };
    });
    tiles
        .iter()
        .filter(|(_, k)| **k != Tile::Wall)
        .map(|((x, y), _)| 100 * y + x)
        .sum::<i64>() as u64
}

pub fn process_part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const EXAMPLE2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE1), 2028);
        assert_eq!(process_part1(EXAMPLE2), 10092);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE2), 10092);
    }
}
