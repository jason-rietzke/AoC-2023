#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let mut sum = 0;
    _input.lines().for_each(|game| {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        let line_content = game.split(": ").collect::<Vec<&str>>()[1];
        let draws = line_content.split("; ").collect::<Vec<&str>>();
        draws.iter().for_each(|draw| {
            let cube_sets = draw.split(", ").collect::<Vec<&str>>();
            cube_sets.iter().for_each(|cube_set| {
                let cube_info = cube_set.split(" ").collect::<Vec<&str>>();
                let amount = cube_info[0].parse::<i32>().unwrap();
                let color = cube_info[1];
                if color == "red" && amount > min_red {
                    min_red = amount;
                } else if color == "green" && amount > min_green {
                    min_green = amount;
                } else if color == "blue" && amount > min_blue {
                    min_blue = amount;
                }
            });
        });
        sum += min_red * min_green * min_blue;
    });
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
