use anyhow::{Context, Result, bail};

/// Parses the input into a vector of total calories per elf.
pub fn parse_elf_calories(input: &str) -> Result<Vec<u32>> {
    // Normalize Windows line endings so splitting on "\n\n" is reliable.
    let normalized = input.replace("\r\n", "\n");

    let calories = normalized
        // Drop trailing whitespace/newlines so we don't accidentally create an extra empty "block".
        .trim_end()
        // Split the input into blocks separated by blank lines.
        .split("\n\n")
        // Map blocks to their summed calories, propagating parse/overflow errors with context.
        .map(|block| -> Result<u32> {
            block
                // Split the block into lines.
                .lines()
                // Ignore empty/whitespace-only lines (defensive; blocks *shouldn't* contain these).
                .filter(|line| !line.trim().is_empty())
                // Parse each line as a u32, attaching a helpful error message on failure.
                .map(|line| {
                    line.trim()
                        .parse::<u32>()
                        .with_context(|| format!("Failed to parse number: {line:?}"))
                })
                // Sum the numbers, failing if the sum would overflow u32.
                .try_fold(0u32, |acc, n| {
                    let n = n?;
                    acc.checked_add(n).context("Calories sum overflow")
                })
        })
        .collect::<Result<Vec<_>>>()?;

    if calories.is_empty() {
        bail!("No calorie blocks found in input");
    }

    Ok(calories)
}

/// Part 1: find the maximum calories carried by any single elf.
pub fn part_one(calories: &[u32]) -> u32 {
    *calories.iter().max().unwrap_or(&0)
}

/// Part 2: find the sum of the top three calorie totals.
pub fn part_two(calories: &[u32]) -> u32 {
    let mut sorted = calories.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    sorted.iter().take(3).copied().sum()
}

fn main() -> Result<()> {
    // Embed the input file at compile time so the solution is a single binary with no runtime I/O.
    let input = include_str!("../input.txt");

    let calories = parse_elf_calories(input).context("Failed to parse calories")?;
    let part1 = part_one(&calories);
    let part2 = part_two(&calories);

    println!("{part1}");
    println!("{part2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_calorie_parsing() -> Result<()> {
        let sample = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

        let got = parse_elf_calories(sample)?;
        assert_eq!(got, vec![6000, 4000, 11000, 24000, 10000]);
        Ok(())
    }

    #[test]
    fn sample_top_three_sum() -> Result<()> {
        let sample = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

        let calories = parse_elf_calories(sample)?;
        let got = part_two(&calories);
        assert_eq!(got, 45000);
        Ok(())
    }
}
