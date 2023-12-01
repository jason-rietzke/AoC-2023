#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let mut result = 0;
    let lines = _input.lines().collect::<Vec<&str>>();
    lines.iter().for_each(|line| {
        let mut number_str = String::new();
        let mut latest_number: char = '0';
        let chars = line.chars().collect::<Vec<char>>();
        chars.iter().for_each(|c| {
            if is_number(c) {
                latest_number = *c;
            }
            if is_number(c) && number_str.len() == 0 {
                number_str.push(latest_number);
            }
        });
        number_str.push(latest_number);
        result += number_str.parse::<i32>().unwrap();
    });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test1.txt");
        assert_eq!(142, process(input)?);
        Ok(())
    }
}

fn is_number(s: &char) -> bool {
    return s.is_digit(10);
}
