#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, Clone)]
enum StoneType {
    Round,
    Cube,
    None,
}

fn print_map(map: &Vec<Vec<StoneType>>) {
    map.iter().for_each(|line| {
        line.iter().for_each(|stone_type| {
            let c = match stone_type {
                StoneType::Round => 'O',
                StoneType::Cube => '#',
                StoneType::None => '.',
            };
            print!("{}", c);
        });
        println!();
    });
}

fn calc_load(map: &Vec<Vec<StoneType>>, direction: &Direction) -> i32 {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut total_load: i32 = 0;
    for y in 0..height {
        for x in 0..width {
            let stone_type = &map[y as usize][x as usize];
            if stone_type == &StoneType::Round {
                match direction {
                    Direction::North => total_load += height - y,
                    Direction::South => total_load += y + 1,
                    Direction::East => total_load += x + 1,
                    Direction::West => total_load += width - x,
                }
            }
        }
    }
    total_load
}

fn can_move(map: &Vec<Vec<StoneType>>, dir: &Direction, x: i32, y: i32) -> bool {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    match dir {
        Direction::North => y - 1 >= 0 && map[(y - 1) as usize][x as usize] == StoneType::None,
        Direction::South => y + 1 < height && map[(y + 1) as usize][x as usize] == StoneType::None,
        Direction::East => x + 1 < width && map[y as usize][(x + 1) as usize] == StoneType::None,
        Direction::West => x - 1 >= 0 && map[y as usize][(x - 1) as usize] == StoneType::None,
    }
}

fn move_stones_to(map: &mut Vec<Vec<StoneType>>, dir: &Direction) -> bool {
    let mut moved = false;
    match dir {
        Direction::North => {
            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    let stone_type = &map[y][x];
                    if stone_type == &StoneType::Round && can_move(map, dir, x as i32, y as i32) {
                        map[y][x] = StoneType::None;
                        map[y - 1][x] = StoneType::Round;
                        moved = true;
                    }
                }
            }
        }
        Direction::South => {
            for y in (0..map.len()).rev() {
                for x in 0..map[0].len() {
                    let stone_type = &map[y][x];
                    if stone_type == &StoneType::Round && can_move(map, dir, x as i32, y as i32) {
                        map[y][x] = StoneType::None;
                        map[y + 1][x] = StoneType::Round;
                        moved = true;
                    }
                }
            }
        }
        Direction::East => {
            for x in (0..map[0].len()).rev() {
                for y in 0..map.len() {
                    let stone_type = &map[y][x];
                    if stone_type == &StoneType::Round && can_move(map, dir, x as i32, y as i32) {
                        map[y][x] = StoneType::None;
                        map[y][x + 1] = StoneType::Round;
                        moved = true;
                    }
                }
            }
        }
        Direction::West => {
            for x in 0..map[0].len() {
                for y in 0..map.len() {
                    let stone_type = &map[y][x];
                    if stone_type == &StoneType::Round && can_move(map, dir, x as i32, y as i32) {
                        map[y][x] = StoneType::None;
                        map[y][x - 1] = StoneType::Round;
                        moved = true;
                    }
                }
            }
        }
    }
    moved
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let lines = _input.lines().collect::<Vec<&str>>();

    let mut map = Vec::<Vec<StoneType>>::new();
    lines.iter().enumerate().for_each(|(y, line)| {
        map.push(Vec::<StoneType>::new());
        line.chars().enumerate().for_each(|(_, c)| {
            let stone_type = match c {
                'O' => StoneType::Round,
                '#' => StoneType::Cube,
                _ => StoneType::None,
            };
            map[y as usize].push(stone_type);
        });
    });

    print_map(&map);

    let mut safety = 0;
    while move_stones_to(&mut map, &Direction::North) && safety < 10000 {
        safety += 1;
    }

    println!("\n{}\n", "=".repeat(map[0].len()));
    print_map(&map);

    let load = calc_load(&map, &Direction::North) as i32;
    println!("load: {}", load);
    Ok(load)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(64, process(input)?);
        Ok(())
    }
}
