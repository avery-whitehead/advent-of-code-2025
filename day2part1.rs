use std::{fs, ops::Range};

use eyre::{eyre, Result};
use fancy_regex::Regex;

fn main() -> Result<()> {
    let re = Regex::new(r"^(\d*)\1{1,}$")?;
    let input = fs::read_to_string("./src/input")?;

    let ids = &input
        .split(",")
        .map(|range| {
            if let Some((left, right)) = range.split_once("-") {
                Ok(left.parse::<usize>()?..right.parse::<usize>()? + 1)
            } else {
                Err(eyre!("Failed to parse range {}", range))
            }
        })
        .fold(Ok(0), |outer_acc: Result<usize>, range| {
            let sum = sum_range_repeated_digits(&re, range?);
            Ok(sum + outer_acc?)
        });

    println!("{:?}", ids);
    Ok(())
}

fn sum_range_repeated_digits(re: &Regex, range: Range<usize>) -> usize {
    range.fold(0, |acc, id| {
        let id_string = id.to_string();
        if re.is_match(&id_string).expect("Failed to execute") {
            acc + id
        } else {
            acc
        }
    })
}
