#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(62, process(input)?);
        Ok(())
    }
}
