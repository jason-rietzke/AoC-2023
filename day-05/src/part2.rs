#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u64> {
    todo!("Implement part 2")
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
