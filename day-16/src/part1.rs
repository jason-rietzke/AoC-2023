#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    ClockwiseMirror,
    CounterClockwiseMirror,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileState {
    Energized,
    DeEnergized,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum MapPrintMode {
    Types,
    State,
}

fn eval_next_pos(x: i32, y: i32, dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn eval_next_dirs(ttype: TileType, dir: Direction) -> Vec<Direction> {
    match ttype {
        TileType::Empty => vec![dir],
        TileType::VerticalSplitter => match dir {
            Direction::Up | Direction::Down => vec![dir],
            _ => vec![Direction::Up, Direction::Down],
        },
        TileType::HorizontalSplitter => match dir {
            Direction::Left | Direction::Right => vec![dir],
            _ => vec![Direction::Left, Direction::Right],
        },
        TileType::ClockwiseMirror => match dir {
            Direction::Up => vec![Direction::Right],
            Direction::Down => vec![Direction::Left],
            Direction::Left => vec![Direction::Down],
            Direction::Right => vec![Direction::Up],
        },
        TileType::CounterClockwiseMirror => match dir {
            Direction::Up => vec![Direction::Left],
            Direction::Down => vec![Direction::Right],
            Direction::Left => vec![Direction::Up],
            Direction::Right => vec![Direction::Down],
        },
    }
}

fn parse_map(input: &str) -> Vec<Vec<(TileState, TileType)>> {
    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines[0].len();

    let mut map = vec![vec![(TileState::DeEnergized, TileType::Empty); width]; lines.len()];
    for row in 0..lines.len() {
        for col in 0..width {
            let c = lines[row].chars().nth(col).unwrap();
            match c {
                '|' => {
                    map[row][col] = (TileState::DeEnergized, TileType::VerticalSplitter);
                }
                '-' => {
                    map[row][col] = (TileState::DeEnergized, TileType::HorizontalSplitter);
                }
                '/' => {
                    map[row][col] = (TileState::DeEnergized, TileType::ClockwiseMirror);
                }
                '\\' => {
                    map[row][col] = (TileState::DeEnergized, TileType::CounterClockwiseMirror);
                }
                _ => {
                    map[row][col] = (TileState::DeEnergized, TileType::Empty);
                }
            }
        }
    }
    map
}

fn print_map(map: &Vec<Vec<(TileState, TileType)>>, mode: MapPrintMode) {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            match mode {
                MapPrintMode::Types => match map[row][col].1 {
                    TileType::Empty => print!("."),
                    TileType::VerticalSplitter => print!("|"),
                    TileType::HorizontalSplitter => print!("-"),
                    TileType::ClockwiseMirror => print!("/"),
                    TileType::CounterClockwiseMirror => print!("\\"),
                },
                MapPrintMode::State => match map[row][col].0 {
                    TileState::Energized => print!("#"),
                    TileState::DeEnergized => print!("."),
                },
            }
        }
        println!();
    }
}

fn energize_map(
    map: &mut Vec<Vec<(TileState, TileType)>>,
    start: (i32, i32, Direction),
    pos_set: &mut Vec<(i32, i32, Direction)>,
) {
    if pos_set.contains(&start) {
        return;
    }
    pos_set.push(start);

    let dir = start.2;
    let ttype = map[start.1 as usize][start.0 as usize].1;

    if map[start.1 as usize][start.0 as usize].0 == TileState::DeEnergized {
        map[start.1 as usize][start.0 as usize] = (
            TileState::Energized,
            map[start.1 as usize][start.0 as usize].1,
        );
    }

    // println!("");
    // print_map(&map, MapPrintMode::State);

    let next_dirs = eval_next_dirs(ttype, dir);
    for next_dir in next_dirs {
        let next_pos = eval_next_pos(start.0, start.1, next_dir);
        if next_pos.0 >= 0
            && next_pos.0 < map[0].len() as i32
            && next_pos.1 >= 0
            && next_pos.1 < map.len() as i32
        {
            energize_map(map, (next_pos.0, next_pos.1, next_dir), pos_set)
        }
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let lines = _input.lines().collect::<Vec<&str>>();
    let width = lines[0].len();

    let mut map = parse_map(_input);
    print_map(&map, MapPrintMode::Types);

    energize_map(&mut map, (0, 0, Direction::Right), &mut Vec::new());

    println!("");
    print_map(&map, MapPrintMode::State);

    let mut energized_tiles_count = 0;
    for row in 0..lines.len() {
        for col in 0..width {
            if map[row][col].0 == TileState::Energized {
                energized_tiles_count += 1;
            }
        }
    }
    Ok(energized_tiles_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(46, process(input)?);
        Ok(())
    }
}
