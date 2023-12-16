#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = include_str!("../test.txt");
        assert_eq!(0, process(input)?);
        Ok(())
    }
}
