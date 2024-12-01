use std::collections::HashMap;


pub fn nums_diff(s: &str) -> Result<u64, anyhow::Error> {
    let (mut left_list, mut right_list) = parse_nums(s)?;

    left_list.sort();
    right_list.sort();

    let result = left_list.iter().zip(right_list.iter()).map(|(&l, &r)| l.abs_diff(r)).sum();

    Ok(result)
}

fn get_num(s: Option<&str>) -> Result<u64, anyhow::Error> {
    Ok(s.unwrap().parse::<u64>()?)
}

pub fn similarity_score(s: &str)  -> Result<u64, anyhow::Error> {
    let (left_list, right_list) = parse_nums(s)?;

    let mut nums_count = HashMap::<u64, u64>::new();
    for num in right_list {
        *nums_count.entry(num).or_default() += 1;
    }

    let result = left_list.iter()
        .map(|&i| i * nums_count.get(&i).cloned().unwrap_or(0))
        .sum();

    Ok(result)
}

pub fn parse_nums(s: &str) -> Result<(Vec<u64>, Vec<u64>), anyhow::Error> {
    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in s.lines() {
        let mut split = line.split_whitespace();
        let lhs = get_num(split.next())?;
        let rhs = get_num(split.next())?;

        left_list.push(lhs);
        right_list.push(rhs);
    }

    Ok((left_list, right_list))
}
