#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes?
    let mut sum = 0;
    let mut line_id = 1;
    _input.lines().for_each(|game| {
        let mut is_possible = true;
        let line_content = game.split(": ").collect::<Vec<&str>>()[1];
        let draws = line_content.split("; ").collect::<Vec<&str>>();
        draws.iter().for_each(|draw| {
            let cube_sets = draw.split(", ").collect::<Vec<&str>>();
            cube_sets.iter().for_each(|cube_set| {
                let cube_info = cube_set.split(" ").collect::<Vec<&str>>();
                let amount = cube_info[0].parse::<i32>().unwrap();
                let color = cube_info[1];
                if color == "red" && amount > 12 {
                    is_possible = false;
                } else if color == "green" && amount > 13 {
                    is_possible = false;
                } else if color == "blue" && amount > 14 {
                    is_possible = false;
                }
            });
        });
        if is_possible {
            sum += line_id;
        }
        line_id += 1;
    });
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
