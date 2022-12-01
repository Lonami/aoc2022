use aoc2022::{Error, Result};

use std::fs;

#[derive(Debug)]
struct Food {
    calories: i32,
}

#[derive(Debug)]
struct Elf {
    food: Vec<Food>,
}

impl Food {
    fn new(calories: i32) -> Self {
        Self { calories }
    }
}

impl Elf {
    fn new() -> Self {
        Self { food: Vec::new() }
    }

    fn total_calories(&self) -> i32 {
        self.food.iter().map(|f| f.calories).sum()
    }
}

fn parse_input(input: &str) -> Result<Vec<Elf>> {
    Ok(input
        .lines()
        .try_fold(vec![Elf::new()], |mut elves, line| {
            if line.is_empty() {
                elves.push(Elf::new())
            } else {
                let calories = line.parse()?;
                elves.last_mut().unwrap().food.push(Food::new(calories));
            }

            Ok::<_, Error>(elves)
        })?
        .into_iter()
        .filter(|elf| !elf.food.is_empty())
        .collect())
}

fn find_most_calories(elves: &[Elf]) -> i32 {
    elves
        .iter()
        .map(|elf| elf.total_calories())
        .max()
        .expect("empty elves list")
}

fn main() -> Result<()> {
    let input = parse_input(&fs::read_to_string("inputs/01/1")?)?;
    println!("Part 1 answer: {}", find_most_calories(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() -> Result<()> {
        let input = parse_input(&fs::read_to_string("inputs/01/1.test")?)?;
        assert_eq!(find_most_calories(&input), 24000);
        Ok(())
    }
}
