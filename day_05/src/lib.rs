
type PageOrder = (u64, u64);
type PrintedPages = Vec<u64>;

pub fn process_middle_page_sum(input: &str) -> u64 {
    let (orders, prints) = input.split_once("\n\n")
        .map(|(first, second)| (parse_orders(first), parse_pages(second)))
        .unwrap();

    let mut sum = 0;
    // O(n^3) this can be optimized
    println!("Ordered:");
    for print in prints {
        
        let ordered = print.iter()
            .enumerate()
            .all(|(i, page)| {
                let rest = &print[i + 1..];
                orders.iter()
                    .filter(|&(_, right)| right == page)
                    .all(|(left, _)| !rest.contains(left))
            });
        if !ordered {
            continue;
        }

        let middle = print[print.len() / 2];
        // println!("{:?} = ({})", print, middle);
        sum += middle;
    }
    sum
}

fn parse_orders(s: &str) -> Vec<PageOrder> {
    s.lines()
        .map(|l| l.split_once('|').unwrap())
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
        .collect()
}

fn parse_pages(s: &str) -> Vec<PrintedPages> {
    s.lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect()
}
