use std::collections::HashSet;
use std::io::{self, Write};

use common::types::Result;
use common::utils::read_lines_from_file;

fn main() -> Result<()> {
    let input = read_lines_from_file("input.txt")?;

    let freq = part1(&input)?;
    writeln!(io::stdout(), "{}", freq)?;

    let freq = part2(&input)?;
    writeln!(io::stdout(), "{}", freq)?;

    Ok(())
}

fn part1(freq_changes: &Vec<String>) -> Result<i32> {
    Ok(freq_changes.iter().fold(0, |acc, f| {
        acc + f.parse::<i32>().expect("should be a number")
    }))
}

fn part2(freq_changes: &Vec<String>) -> Result<i32> {
    let mut hashset: HashSet<i32> = HashSet::from([0]);
    let mut sum = 0;
    'outer: loop {
        for f in freq_changes {
            sum += f.parse::<i32>().expect("should be a number");
            if hashset.contains(&sum) {
                break 'outer;
            }
            hashset.insert(sum);
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let _ = part1(&vec!["1".to_string(), "-1".to_string(), "2".to_string()]);
    }
}
