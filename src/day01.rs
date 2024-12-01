use std::collections::{HashMap, HashSet};

use aoc2024::Day;
use color_eyre::{eyre::ContextCompat, Result};

inventory::submit! {
    Day::new(1, part1, part2)
}

fn part1(input: &str) -> Result<String> {
    let pairs = input.lines().map(parse_line).collect::<Result<Vec<_>>>()?;

    let (mut list1, mut list2): (Vec<i64>, Vec<i64>) = pairs.into_iter().unzip();
    list1.sort();
    list2.sort();

    let sum: i64 = list1
        .into_iter()
        .zip(list2)
        .map(|(v1, v2)| (v1 - v2).abs())
        .sum();

    Ok(format!("{sum}"))
}

fn part2(input: &str) -> Result<String> {
    let pairs = input.lines().map(parse_line).collect::<Result<Vec<_>>>()?;
    let (left, right): (Vec<i64>, Vec<i64>) = pairs.into_iter().unzip();

    // HashSet of all the entries in the right list
    let right_set = right.iter().copied().collect::<HashSet<_>>();

    // Count how many times each entry in the right list occurs
    let counts = right
        .iter()
        .fold(HashMap::<i64, i64>::new(), |mut counts, n| {
            if right_set.contains(n) {
                *(counts.entry(*n).or_default()) += 1;
            }
            counts
        });

    // Compute the similarity score
    let sum = left
        .into_iter()
        .map(|n| counts.get(&n).copied().unwrap_or_default() * n)
        .sum::<i64>();

    Ok(format!("{sum}"))
}

fn parse_line(line: &str) -> Result<(i64, i64)> {
    let mut iter = line.split_whitespace();
    let left = iter.next().context("Invalid input")?.parse()?;
    let right = iter.next().context("Invalid input")?.parse()?;

    Ok((left, right))
}
