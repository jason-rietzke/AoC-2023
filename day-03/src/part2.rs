use std::clone::Clone;
use std::marker::Copy;

#[derive(Copy)]
struct NumberBlock {
    number: i32,
    row: i32,
    col_start: i32,
    col_end: i32,
    is_part_number: bool,
}

struct Symbol {
    is_gear: bool,
    row: i32,
    col: i32,
    adjacent_number_blocks: Vec<NumberBlock>,
}

impl NumberBlock {
    fn is_adjacent(&self, pos: (i32, i32)) -> bool {
        let boundaries = (
            (self.row - 1, self.col_start - 1),
            (self.row + 1, self.col_end + 1),
        );
        if pos.0 >= boundaries.0 .0 && pos.0 <= boundaries.1 .0 {
            if pos.1 >= boundaries.0 .1 && pos.1 <= boundaries.1 .1 {
                return true;
            }
        }
        false
    }
}
impl Clone for NumberBlock {
    fn clone(&self) -> Self {
        *self
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let mut sum = 0;
    let lines = _input.lines().collect::<Vec<&str>>();
    let row_count = lines.len();
    let col_count = lines[0].len();
    let mut number_blocks = Vec::new();
    let mut symbols = Vec::new();

    for row_index in 0..row_count {
        let mut num_block_chars = Vec::new();
        for col_index in 0..col_count {
            let c = lines[row_index].chars().nth(col_index).unwrap();
            if !c.is_digit(10) {
                if c != '.' {
                    let symbol = Symbol {
                        is_gear: c == '*',
                        row: row_index as i32,
                        col: col_index as i32,
                        adjacent_number_blocks: Vec::new(),
                    };
                    symbols.push(symbol);
                }
                add_number_block(
                    &mut num_block_chars,
                    &mut number_blocks,
                    row_index,
                    col_index,
                );
                continue;
            } else {
                num_block_chars.push(c);
            }
        }
        add_number_block(
            &mut num_block_chars,
            &mut number_blocks,
            row_index,
            col_count,
        );
    }

    for block in &mut number_blocks {
        for symbol in &mut symbols {
            if block.is_adjacent((symbol.row, symbol.col)) {
                block.is_part_number = true;
                if symbol.is_gear {
                    symbol.adjacent_number_blocks.push(*block);
                }
                break;
            }
        }
    }

    for symbol in &symbols {
        if !symbol.is_gear {
            continue;
        }
        if symbol.adjacent_number_blocks.len() != 2 {
            continue;
        }
        sum += symbol.adjacent_number_blocks[0].number * symbol.adjacent_number_blocks[1].number;
    }

    Ok(sum)
}

fn add_number_block(
    num_block_chars: &mut Vec<char>,
    number_blocks: &mut Vec<NumberBlock>,
    row_index: usize,
    col_index: usize,
) {
    if num_block_chars.len() > 0 {
        let num_block_str = num_block_chars.iter().collect::<String>();
        let num_block = NumberBlock {
            number: num_block_str.parse::<i32>().unwrap(),
            row: row_index as i32,
            col_start: col_index as i32 - num_block_chars.len() as i32,
            col_end: col_index as i32 - 1,
            is_part_number: false,
        };
        number_blocks.push(num_block);
        num_block_chars.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(467835, process(input)?);
        Ok(())
    }
}
