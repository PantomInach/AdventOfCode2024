use std::collections::HashMap;

use itertools::Itertools;

type VarMap = HashMap<String, i32>;

enum Operation {
    OR(i32, i32, i32),
    XOR(i32, i32, i32),
    AND(i32, i32, i32),
    Set(i32, bool),
}
impl Operation {
    fn add(&self, sat: &mut cat_solver::Solver) {
        match self {
            Operation::OR(l1, l2, out) => {
                sat.add_clause(vec![-*l1, *out]);
                sat.add_clause(vec![-*l2, *out]);
                sat.add_clause(vec![*l1, *l2, -*out]);
            }
            Operation::XOR(l1, l2, out) => {
                sat.add_clause(vec![-*l1, *l2, *out]);
                sat.add_clause(vec![*l1, -*l2, *out]);
                sat.add_clause(vec![-*l1, -*l2, -*out]);
                sat.add_clause(vec![*l1, *l2, -*out]);
            }
            Operation::AND(l1, l2, out) => {
                sat.add_clause(vec![-l1, -l2, *out]);
                sat.add_clause(vec![*l1, -*out]);
                sat.add_clause(vec![*l2, -*out]);
            }
            Operation::Set(out, b) => {
                if *b {
                    sat.add_clause(vec![*out]);
                } else {
                    sat.add_clause(vec![-*out]);
                }
            }
        }
    }
}

fn parse_input(input: &str) -> (VarMap, Vec<Operation>) {
    let mut lines = input.lines();
    let mut var_map: VarMap = HashMap::new();
    let mut operations: Vec<Operation> = vec![];
    let mut i = 1;
    loop {
        let l = lines.next().unwrap();
        if l.is_empty() {
            break;
        }
        let (lit, val) = l.split_once(": ").unwrap();
        var_map.insert(lit.to_string(), i);
        operations.push(Operation::Set(i, val.parse::<u8>().unwrap() == 1));
        i += 1;
    }
    while let Some(l) = lines.next() {
        let (front, out_str) = l.split_once(" -> ").unwrap();
        if !var_map.contains_key(out_str) {
            var_map.insert(out_str.to_string(), i);
            i += 1;
        }
        let mut front_it = front.split_whitespace();
        let lit1 = front_it.next().unwrap();
        if !var_map.contains_key(lit1) {
            var_map.insert(lit1.to_string(), i);
            i += 1;
        }
        let op = front_it.next().unwrap();
        let lit2 = front_it.next().unwrap();
        if !var_map.contains_key(lit2) {
            var_map.insert(lit2.to_string(), i);
            i += 1;
        }

        let l1 = var_map.get(lit1).unwrap();
        let l2 = var_map.get(lit2).unwrap();
        let lout = var_map.get(out_str).unwrap();

        operations.push(match op {
            "OR" => Operation::OR(*l1, *l2, *lout),
            "AND" => Operation::AND(*l1, *l2, *lout),
            "XOR" => Operation::XOR(*l1, *l2, *lout),
            _ => unreachable!(),
        });
    }
    (var_map, operations)
}

pub fn process_part1(input: &str) -> u64 {
    let (var_map, operations) = parse_input(input);
    let mut sat: cat_solver::Solver = cat_solver::Solver::new();
    operations.iter().for_each(|op| op.add(&mut sat));
    sat.solve();
    var_map
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, lit)| (k, sat.value(*lit).unwrap()))
        .sorted_by_key(|(k, _)| *k)
        .rev()
        .fold(0_u64, |acc, (k, lit)| (acc * 2) + lit as u64)
}

pub fn process_part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), 2024);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE), "co,de,ka,ta");
    }
}
