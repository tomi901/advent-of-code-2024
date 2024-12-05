use std::{cmp, collections::{HashMap, HashSet}};

type PageOrder = (u64, u64);
type PrintedPages = Vec<u64>;

#[derive(Debug, Clone)]
struct PagePriorities(HashMap<u64, u64>);

impl PagePriorities {
    pub fn new(orders: &[PageOrder]) -> Self {
        let mut priorities = Self(HashMap::new());

        let root_pages = {
            let mut pages = orders.iter()
                .map(|&(left, _)| left)
                .collect::<HashSet<_>>();
            for (_, right) in orders {
                pages.remove(right);
            }
            pages
        };

        for page in root_pages {
            priorities.set_priority_recursively(page, 0, orders);
        }
        priorities
    }

    pub fn get(&self, page: &u64) -> Option<&u64> {
        self.0.get(&page)
    }

    fn set_priority_recursively(&mut self, page: u64, priority: u64, orders: &[PageOrder]) {
        let entry = self.0.entry(page).or_default();
        *entry = cmp::max(*entry, priority);
        for &(_, right) in orders.iter().filter(|&(left, _)| left == &page) {
            self.set_priority_recursively(right, priority + 1, orders);
        }
    }
}

pub fn process_middle_page_sum(input: &str) -> u64 {
    let (orders, prints) = parse_info(input);

    let mut sum = 0;
    let mut count = 0;
    // O(n^3) this can be optimized
    // println!("Ordered:");
    for print in &prints {
        
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
        count += 1;
    }
    println!("Ordered count: {}/{}", count, prints.len());
    sum
}

pub fn reordered_middle_page_sum(input: &str) -> u64 {
    let (orders, mut prints) = parse_info(input);

    let mut sum = 0;
    for print in &mut prints {
        let pages_set = print.iter().cloned().collect::<HashSet<_>>();
        let relevant_orders = orders.iter()
            .filter(|&(before, after)| pages_set.contains(before) && pages_set.contains(after))
            .cloned()
            .collect::<Vec<_>>();

        let priorities = PagePriorities::new(&relevant_orders);
        let original = print.clone();

        print.sort_by_key(|page| priorities.get(page));
        if original.eq(print) {
            continue;
        }

        sum += print[print.len() / 2];
        // println!("{sum}");
    }
    sum
}

fn parse_info(s: &str) -> (Vec<PageOrder>, Vec<PrintedPages>) {
    s.split_once("\n\n")
        .map(|(first, second)| (parse_orders(first), parse_pages(second)))
        .unwrap()
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
