
pub fn count_safe(input: &str) -> usize {
    input.lines().filter(|&l| line_is_safe(l)).count()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    None,
    Increasing,
    Decreasing,
}

fn line_is_safe(line: &str) -> bool {
    let mut kind = Kind::None;
    let mut split = line.split_whitespace();
    let mut previous: i64 = split.next().unwrap().parse().unwrap();
    for cur in split {
        let cur = cur.parse::<i64>().unwrap();
        if !(1..=3).contains(&cur.abs_diff(previous)) {
            return false;
        }

        match kind {
            Kind::None => match cur.cmp(&previous) {
                std::cmp::Ordering::Less => kind = Kind::Decreasing,
                std::cmp::Ordering::Equal => unreachable!(),
                std::cmp::Ordering::Greater => kind = Kind::Increasing,
            },
            Kind::Increasing => if cur <= previous {
                return false;
            },
            Kind::Decreasing => if cur >= previous {
                return false;
            },
        }

        previous = cur;
    }
    true
}
