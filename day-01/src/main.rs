use anyhow::{Context, Result};

/// Finds the maximum calories carried by any single elf.
///
/// The input consists of blocks of numbers separated by blank lines.
/// Each block sum is an elf's total calories; we return the maximum of those sums.
pub fn find_max_calories(input: &str) -> Result<u32> {
    // Normalize Windows line endings so splitting on "\n\n" is reliable.
    let normalized = input.replace("\r\n", "\n");

    normalized
        // Drop trailing whitespace/newlines so we don't accidentally create an extra empty "block".
        .trim_end()
        // Split the input into blocks separated by blank lines.
        .split("\n\n")
        // Fold blocks into a running max, propagating parse/overflow errors with context.
        .try_fold(None::<u32>, |current_max, block| -> Result<Option<u32>> {
            let block_sum = block
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
                })?;

            Ok(Some(match current_max {
                Some(m) => m.max(block_sum),
                None => block_sum,
            }))
        })?
        .context("No calorie blocks found in input")
}

fn main() -> Result<()> {
    // Embed the input file at compile time so the solution is a single binary with no runtime I/O.
    let input = include_str!("../input.txt");

    let max = find_max_calories(input).context("Failed to compute max calories")?;
    println!("{max}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_max_calories() -> Result<()> {
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

        let got = find_max_calories(sample)?;
        assert_eq!(got, 24000);
        Ok(())
    }
}
