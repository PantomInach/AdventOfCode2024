#[derive(PartialEq, Eq, Clone, Copy)]
enum Xmas {
    X,
    M,
    A,
    S,
    Other,
}

#[inline]
fn xmas_like(comp: &[Xmas]) -> bool {
    comp == [Xmas::X, Xmas::M, Xmas::A, Xmas::S] || comp == [Xmas::S, Xmas::A, Xmas::M, Xmas::X]
}

fn count_hori(line: &Vec<Xmas>) -> u64 {
    line.as_slice().windows(4).filter(|a| xmas_like(a)).count() as u64
}

fn count_diag_down(arr: &[Vec<Xmas>]) -> u64 {
    let mut count: u64 = 0;
    for i in 0..arr.len() - 3 {
        let l0 = arr.get(i).unwrap();
        let l1 = arr.get(i + 1).unwrap();
        let l2 = arr.get(i + 2).unwrap();
        let l3 = arr.get(i + 3).unwrap();
        for j in 0..arr.first().unwrap().len() - 3 {
            if xmas_like(&[
                *l0.get(j).unwrap(),
                *l1.get(j + 1).unwrap(),
                *l2.get(j + 2).unwrap(),
                *l3.get(j + 3).unwrap(),
            ]) {
                count += 1;
            }
        }
    }
    count
}

fn transpose(arr: &[Vec<Xmas>]) -> Vec<Vec<Xmas>> {
    let mut arr_new: Vec<Vec<Xmas>> =
        vec![vec![Xmas::Other; arr.first().unwrap().len()]; arr.len()];
    arr.iter().enumerate().for_each(|(i, l)| {
        l.iter().enumerate().for_each(|(j, x)| {
            arr_new[j][i] = *x;
        })
    });
    arr_new
}

fn flip(arr: &[Vec<Xmas>]) -> Vec<Vec<Xmas>> {
    arr.iter()
        .map(|l| {
            let mut ll = l.clone();
            ll.reverse();
            ll
        })
        .collect()
}

pub fn process_part1(input: &str) -> u64 {
    let arr: Vec<Vec<Xmas>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'X' => Xmas::X,
                    'M' => Xmas::M,
                    'A' => Xmas::A,
                    'S' => Xmas::S,
                    _ => Xmas::Other,
                })
                .collect::<Vec<Xmas>>()
        })
        .collect();
    let mut total: u64 = 0;
    total += arr.iter().map(count_hori).sum::<u64>(); // horizontal
    total += transpose(&arr).iter().map(count_hori).sum::<u64>(); // vertical
    total += count_diag_down(&arr); // diag right down
    total += count_diag_down(&flip(&arr)); // diag left down
    total
}

fn is_x_max(x: &[[Xmas; 3]; 3]) -> bool {
    if x[1][1] != Xmas::A {
        false
    } else if x[0][0] == Xmas::M && x[2][2] == Xmas::S || x[0][0] == Xmas::S && x[2][2] == Xmas::M {
        x[0][2] == Xmas::M && x[2][0] == Xmas::S || x[0][2] == Xmas::S && x[2][0] == Xmas::M
    } else {
        false
    }
}

pub fn process_part2(input: &str) -> u64 {
    let arr: Vec<Vec<Xmas>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'X' => Xmas::X,
                    'M' => Xmas::M,
                    'A' => Xmas::A,
                    'S' => Xmas::S,
                    _ => Xmas::Other,
                })
                .collect::<Vec<Xmas>>()
        })
        .collect();
    let mut total: u64 = 0;
    for i in 0..arr.len() - 2 {
        for j in 0..arr.first().unwrap().len() - 2 {
            if is_x_max(&[
                [
                    *arr.get(i).unwrap().get(j).unwrap(),
                    *arr.get(i).unwrap().get(j + 1).unwrap(),
                    *arr.get(i).unwrap().get(j + 2).unwrap(),
                ],
                [
                    *arr.get(i + 1).unwrap().get(j).unwrap(),
                    *arr.get(i + 1).unwrap().get(j + 1).unwrap(),
                    *arr.get(i + 1).unwrap().get(j + 2).unwrap(),
                ],
                [
                    *arr.get(i + 2).unwrap().get(j).unwrap(),
                    *arr.get(i + 2).unwrap().get(j + 1).unwrap(),
                    *arr.get(i + 2).unwrap().get(j + 2).unwrap(),
                ],
            ]) {
                total += 1;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(process_part1(input), 18);
    }

    #[test]
    fn test_process_part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(process_part2(input), 9)
    }
}
