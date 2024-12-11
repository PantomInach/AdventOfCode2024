/// Inefficient Datastructure.
/// Rather use a LinkedList and store free and used blocks seperated.
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, Copy)]
struct Block {
    size: u64,
    id: Option<u16>,
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.size == 0 {
                "#".to_string()
            } else {
                self.id
                    .map(|id| id.to_string())
                    .unwrap_or(".".to_string())
                    .repeat(self.size as usize)
            }
        )
    }
}

fn fancy_filesystem(filesystem: &[Block]) -> String {
    filesystem.iter().map(|b| b.to_string()).collect::<String>()
}

fn parse_input(input: &str) -> Vec<Block> {
    input
        .trim()
        .char_indices()
        .filter_map(|(i, c)| match (i, c) {
            (_, '0') if i % 2 == 0 => unreachable!(),
            (_, '0') if i % 2 == 1 => None,
            (_, _) if i % 2 == 0 => Some(Block {
                size: c.to_string().parse::<u64>().unwrap(),
                id: Some(i as u16 / 2),
            }),
            _ => Some(Block {
                size: c.to_string().parse::<u64>().unwrap(),
                id: None,
            }),
        })
        .collect()
}

fn reformat(filesystem: &mut Vec<Block>) {
    while let Some(pos) = filesystem.iter().position(|b| b.id.is_none()) {
        // println!("{}", fancy_filesystem(filesystem));
        if let Some(block) = filesystem.pop().and_then(|b| b.id.is_some().then_some(b)) {
            let free_block_size = filesystem.get(pos).unwrap().size;
            if free_block_size == 0 {
                filesystem.remove(pos);
            } else if free_block_size > block.size {
                filesystem.insert(
                    pos,
                    Block {
                        size: block.size,
                        id: block.id,
                    },
                );
                filesystem.get_mut(pos + 1).unwrap().size -= block.size;
            } else if free_block_size == block.size {
                filesystem.get_mut(pos).unwrap().id = block.id;
            } else {
                filesystem.get_mut(pos).unwrap().id = block.id;
                filesystem.push(Block {
                    size: block.size - free_block_size,
                    id: block.id,
                })
            }
        }
    }
}

fn checksum(filesystem: &[Block]) -> u64 {
    let mut common_position: u64 = 0;
    filesystem
        .iter()
        .map(|b| {
            common_position += b.size;
            (common_position - b.size..common_position)
                .map(|i| i * b.id.unwrap_or(0) as u64)
                .sum::<u64>()
        })
        .sum()
}

pub fn process_part1(input: &str) -> u64 {
    let filesystem = &mut parse_input(input);
    reformat(filesystem);
    checksum(filesystem)
}

fn defragment(filesystem: &mut Vec<Block>) {
    let mut tried_move_files: HashSet<u16> = HashSet::new();
    while let Some((file_pos, block)) = filesystem
        .iter()
        .enumerate()
        .rev()
        .find(|(_, b)| b.id.is_some_and(|id| !tried_move_files.contains(&id)))
        .map(|(i, b)| (i, b.clone()))
    {
        // println!("{}", fancy_filesystem(filesystem));
        tried_move_files.insert(block.id.unwrap());
        if let Some((pos, free_block)) = filesystem
            .iter()
            .enumerate()
            .find(|(_, b)| b.id.is_none() && b.size >= block.size)
        {
            if file_pos < pos {
                continue;
            } else if free_block.size == block.size {
                filesystem.get_mut(pos).unwrap().id = block.id;
                filesystem.get_mut(file_pos).unwrap().id = None;
            } else {
                filesystem.insert(
                    pos,
                    Block {
                        size: block.size,
                        id: block.id,
                    },
                );
                filesystem.get_mut(pos + 1).unwrap().size -= block.size;
                filesystem.get_mut(file_pos + 1).unwrap().id = None;
            }
        }
    }
}

pub fn process_part2(input: &str) -> u64 {
    let filesystem = &mut parse_input(input);
    defragment(filesystem);
    checksum(filesystem)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), 1928);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE), 2858)
    }
}
