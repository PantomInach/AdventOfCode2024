use std::collections::VecDeque;

type Field = Vec<Vec<Entry>>;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn populate_field(field: &mut Field, cords: &[(usize, usize)], n: usize) {
    cords.iter().take(n).for_each(|(x, y)| {
        field
            .get_mut(*y)
            .map(|l| l.get_mut(*x).map(|e| *e = Entry::Blocked));
    });
}

#[derive(Clone, Eq, PartialEq)]
enum Entry {
    Blocked,
    Unvisited,
    Dist(u64),
}

fn set(x: i32, y: i32, dist: u64, field: &mut Field) {
    if let Some(l) = field.get_mut(y as usize) {
        if let Some(e) = l.get_mut(x as usize) {
            match e {
                Entry::Blocked => (),
                Entry::Unvisited => *e = Entry::Dist(dist),
                Entry::Dist(d) if *d < dist => *e = Entry::Dist(dist),
                _ => (),
            }
        }
    }
}

fn get_dist(x: i32, y: i32, field: &Field) -> Option<u64> {
    if let Some(Some(e)) = field.get(y as usize).map(|l| l.get(x as usize)) {
        match e {
            Entry::Dist(dist) => Some(*dist),
            _ => None,
        }
    } else {
        None
    }
}

fn walkable(x: i32, y: i32, dist: u64, field: &Field) -> bool {
    if let Some(Some(e)) = field.get(y as usize).map(|l| l.get(x as usize)) {
        match e {
            Entry::Unvisited => true,
            Entry::Dist(d) if d > &dist => true,
            _ => false,
        }
    } else {
        false
    }
}

#[inline]
fn neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn bfs(field: &mut Field, field_size: usize) -> u64 {
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_front((0, 0));
    set(0, 0, 0, field);
    while let Some((x, y)) = queue.pop_front() {
        let dist = get_dist(x, y, field).unwrap();
        if x + 1 == field_size as i32 && y + 1 == field_size as i32 {
            return dist;
        }
        if x < 0 || y < 0 || x > field_size as i32 || y > field_size as i32 {
            continue;
        }
        neighbors(x, y).into_iter().for_each(|(xx, yy)| {
            if walkable(xx, yy, dist + 1, field) {
                set(xx, yy, dist + 1, field);
                queue.push_back((xx, yy));
            }
        });
    }
    0
}

pub fn process_part1(input: &str) -> u64 {
    let mut field: Field = vec![vec![Entry::Unvisited; 71]; 71];
    populate_field(&mut field, &parse_input(input), 1024);
    bfs(&mut field, 71)
}

pub fn process_part2(input: &str) -> String {
    let mut n = 0;
    let cords = parse_input(input);
    loop {
        let mut field: Field = vec![vec![Entry::Unvisited; 71]; 71];
        populate_field(&mut field, &cords, n);
        if bfs(&mut field, 71) == 0 {
            let (x, y) = cords.get(n - 1).unwrap();
            return format!("{},{}", x, y);
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_process_part1() {
        let mut field: Field = vec![vec![Entry::Unvisited; 7]; 7];
        populate_field(&mut field, &parse_input(EXAMPLE), 12);
        assert_eq!(bfs(&mut field, 7), 22)
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE), "6,1");
    }
}
