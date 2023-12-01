#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let mut result = 0;
    let lines = _input.lines().collect::<Vec<&str>>();
    lines.iter().for_each(|line| {
        let mut number_str = String::new();
        let mut latest_number: char = '0';
        let mut line = *line;
        while line.len() > 0 {
            let res = slice_and_return_number(line);
            let number = res.0;
            line = res.1;
            if number > -1 {
                latest_number = number.to_string().chars().last().unwrap();
            }
            if number_str.len() == 0 && number > -1 {
                number_str.push_str(&number.to_string());
            }
        }
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
        let input = include_str!("../test2.txt");
        assert_eq!(281, process(input)?);
        Ok(())
    }
}

fn slice_and_return_number(input: &str) -> (i32, &str) {
    let valid_substrings = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    let mut result = -1;
    let mut input = input;
    valid_substrings.iter().for_each(|substring| {
        if result > -1 {
            return;
        }
        if input.starts_with(substring) {
            result = str_to_i32(substring);
            // input = input.strip_prefix(substring).unwrap();
            input = input.split_at(1).1;
        }
    });
    if result == -1 {
        input = input.split_at(1).1;
    }
    return (result, input);
}

fn str_to_i32(s: &str) -> i32 {
    match s {
        "0" => 0,
        "zero" => 0,
        "1" => 1,
        "one" => 1,
        "2" => 2,
        "two" => 2,
        "3" => 3,
        "three" => 3,
        "4" => 4,
        "four" => 4,
        "5" => 5,
        "five" => 5,
        "6" => 6,
        "six" => 6,
        "7" => 7,
        "seven" => 7,
        "8" => 8,
        "eight" => 8,
        "9" => 9,
        "nine" => 9,
        _ => -1,
    }
}
