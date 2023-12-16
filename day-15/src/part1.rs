fn hash(input: &str) -> i32 {
    let mut hash_value = 0;
    for c in input.chars() {
        hash_value += c as i32;
        hash_value *= 17;
        hash_value %= 256;
    }
    hash_value
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let sequences = _input.split(",");
    let mut sum = 0;
    for sequence in sequences {
        let hash_value = hash(sequence);
        sum += hash_value;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(1320, process(input)?);
        Ok(())
    }
}
