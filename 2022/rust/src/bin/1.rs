use std::{fs, io::Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("tmp/1.in")?;

    println!("PART 1: {}", part_1(&input)?);
    println!("PART 2: {}", part_2(&input)?);

    Ok(())
}

fn part_1(input: &str) -> Result<i64> {
    let mut max_sum = 0_i64;
    let mut cur_sum = 0_i64;
    let mut cur_num = "".to_string();
    let mut is_prev_newline = false;
    for c in input.chars() {
        if c == '\n' {
            if is_prev_newline {
                max_sum = max_sum.max(cur_sum);
                cur_sum = 0;
                continue;
            }
            is_prev_newline = true;
            cur_sum += cur_num.parse::<i64>().unwrap();
            cur_num.clear();
        } else {
            is_prev_newline = false;
            cur_num.push(c);
        }
    }
    if cur_sum != 0 {
        max_sum = max_sum.max(cur_sum);
    }
    Ok(max_sum)
}

fn part_2(input: &str) -> Result<i64> {
    let mut maxs = [0_i64, 0_i64, 0_i64];
    let mut cur_sum = 0_i64;
    let mut cur_num = "".to_string();
    let mut is_prev_newline = false;
    for c in input.chars() {
        if c == '\n' {
            if is_prev_newline {
                let mut temp = cur_sum;
                (0..maxs.len()).for_each(|i| {
                    if temp > maxs[i] {
                        (temp, maxs[i]) = (maxs[i], temp);
                    }
                });
                cur_sum = 0;
                continue;
            }
            is_prev_newline = true;
            cur_sum += cur_num.parse::<i64>().unwrap();
            cur_num.clear();
        } else {
            is_prev_newline = false;
            cur_num.push(c);
        }
    }
    if cur_sum != 0 {
        let mut temp = cur_sum;
        (0..maxs.len()).for_each(|i| {
            if temp > maxs[i] {
                (temp, maxs[i]) = (maxs[i], temp);
            }
        });
    }
    Ok(maxs.iter().sum())
}
