#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u64> {
    let blocks = _input.split("\n\n").collect::<Vec<_>>();
    let seeds = blocks[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let seed2soil = parse_block(blocks[1]);
    let soil2fert = parse_block(blocks[2]);
    let fert2water = parse_block(blocks[3]);
    let water2light = parse_block(blocks[4]);
    let light2temp = parse_block(blocks[5]);
    let temp2hum = parse_block(blocks[6]);
    let hum2loc = parse_block(blocks[7]);

    let locations = seeds
        .iter()
        .map(|x| eval_map(*x, seed2soil.clone()))
        .map(|x| eval_map(x, soil2fert.clone()))
        .map(|x| eval_map(x, fert2water.clone()))
        .map(|x| eval_map(x, water2light.clone()))
        .map(|x| eval_map(x, light2temp.clone()))
        .map(|x| eval_map(x, temp2hum.clone()))
        .map(|x| eval_map(x, hum2loc.clone()))
        .collect::<Vec<_>>();

    println!("{:?}", locations);

    Ok(*locations.iter().min().unwrap())
}

fn parse_block(block: &str) -> Vec<(u64, u64, u64)> {
    let mut result: Vec<(u64, u64, u64)> = Vec::new();
    block
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .for_each(|line| {
            if !line.contains(":") {
                let nums = line
                    .split(" ")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                result.push((nums[0], nums[1], nums[2]));
            }
        });
    result
}

fn eval_map(value: u64, map: Vec<(u64, u64, u64)>) -> u64 {
    for (dest, src, len) in map.iter() {
        if value >= *src && value <= src + len {
            return *dest + value - *src;
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(35, process(input)?);
        Ok(())
    }
}
