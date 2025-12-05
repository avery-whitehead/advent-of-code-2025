use std::fs;

use eyre::{Ok, OptionExt, Result};

#[derive(Debug)]
struct Battery {
    bank_idx: usize,
    jolt: char,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("./src/input")?;
    let bank_len = input.lines().nth(0).ok_or_eyre("")?.len();

    let sum = input
        .lines()
        //.map(bank_to_int)
        .fold(0, |acc, line| {
            let bank = line.chars().collect();

            let best = get_largest_in_range(&bank, 0, bank_len - 1);
            let next = get_largest_in_range(&bank, best.bank_idx + 1, bank_len);

            let joltage = format!("{}{}", best.jolt, next.jolt)
                .parse::<usize>()
                .expect("");

            acc + joltage
        });

    println!("{}", sum);
    Ok(())
}

fn get_largest_in_range(bank: &Vec<char>, start_idx: usize, end_idx: usize) -> Battery {
    bank.iter()
        .skip(start_idx)
        .enumerate()
        .take_while(|(idx, _)| *idx < end_idx)
        .fold(
            Battery {
                bank_idx: 0,
                jolt: '0',
            },
            |max, (idx, jolt)| {
                if jolt.to_digit(10).expect("") > max.jolt.to_digit(10).expect("") {
                    Battery {
                        bank_idx: idx,
                        jolt: *jolt,
                    }
                } else {
                    max
                }
            },
        )
}
