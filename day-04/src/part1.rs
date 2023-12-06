use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let mut sum = 0;
    _input.lines().for_each(|line| {
        let card = line.split(": ").collect::<Vec<&str>>()[1];
        let split: (&str, &str) = card.split(" | ").collect_tuple().unwrap();
        let winning_nums = split
            .0
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let drawn_nums = split
            .1
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut match_value = 0;
        for num in drawn_nums {
            if winning_nums.contains(&num) {
                if match_value == 0 {
                    match_value = 1;
                } else {
                    match_value *= 2;
                }
            }
        }
        sum += match_value;
    });
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(13, process(input)?);
        Ok(())
    }
}
