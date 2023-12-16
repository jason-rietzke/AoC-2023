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
    let mut boxes = vec![Vec::<(&str, i32)>::new(); 256];
    for sequence in sequences {
        let label = sequence.split(|c| c == '=' || c == '-').next().unwrap();
        let box_index = hash(label) as usize;
        if sequence.contains("=") {
            let focal_length = sequence.split("=").last().unwrap().parse::<i32>().unwrap();
            let existing_index = boxes[box_index]
                .iter()
                .position(|(l, _)| l == &label)
                .unwrap_or(usize::MAX);
            if existing_index == usize::MAX {
                boxes[box_index].push((label, focal_length));
            } else {
                boxes[box_index][existing_index] = (label, focal_length);
            }
        } else {
            boxes[box_index].retain(|(l, _)| l != &label);
        }
    }

    let mut sum: i32 = 0;
    for box_index in 0..boxes.len() {
        let box_value = 1 + (box_index as i32);
        for lense_index in 0..boxes[box_index].len() {
            let lense_value = ((lense_index as i32) + 1) * boxes[box_index][lense_index].1;
            sum += box_value * lense_value;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(145, process(input)?);
        Ok(())
    }
}
