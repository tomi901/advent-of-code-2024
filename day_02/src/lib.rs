use std::num::ParseIntError;

pub struct Count {
    pub count: usize,
    pub total: usize,
}

pub fn count_safe(input: &str) -> Result<Count, ParseIntError> {
    let reports = input.lines()
        .map(|l| parse_report(l))
        .collect::<Result<Vec<_>, _>>()?;
    let count = reports.iter()
        .filter(|&r| report_is_safe(r.iter().cloned()))
        .count();
    Ok(Count {
        count,
        total: reports.len(),
    })
}

pub fn count_safe_with_tolerance(input: &str) -> Result<Count, ParseIntError> {
    let reports = input.lines()
        .map(|l| parse_report(l))
        .collect::<Result<Vec<_>, _>>()?;
    let count = reports.iter()
        .filter(|r| report_is_safe_tolerant(r))
        .count();
    Ok(Count {
        count,
        total: reports.len(),
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    None,
    Increasing,
    Decreasing,
}

fn parse_report(line: &str) -> Result<Vec<i64>, ParseIntError> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>())
        .collect::<Result<_, _>>()
}

fn report_is_safe(mut report: impl Iterator<Item = i64>) -> bool {
    let mut kind = Kind::None;
    let mut previous: i64 = report.next().unwrap();
    for cur in report {
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

fn report_is_safe_tolerant(report: &[i64]) -> bool {
    if report_is_safe(report.iter().cloned()) {
        return true;
    }

    for filter_i in 0..report.len() {
        let modified_report = report.iter()
            .enumerate()
            .filter(|(i, _)| *i != filter_i)
            .map(|(_, num)| *num);
        if report_is_safe(modified_report) {
            return true;
        }
    }
    false
}
