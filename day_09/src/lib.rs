
#[derive(Debug, Clone, Copy)]
struct ID(u64);

#[derive(Debug, Clone)]
struct Block {
    id: ID,
    length: u8,
}

impl ID {
    const EMPTY: Self = Self(0);

    fn is_free_space(&self) -> bool {
        self.0 == 0
    }

    fn actual_id(&self) -> Option<u64> {
        if !self.is_free_space() {
            Some(self.0 - 1)
        } else {
            None
        }
    }
}

pub fn get_disk_checksum(input: &str) -> u64 {
    let layout_iter = get_file_layout(input.trim());
    // println!("{:?}", layout);
    let uncompressed = get_uncompressed_layout(layout_iter);
    // println!("Uncompressed has length {}", uncompressed.len());
    calculate_checksum(&uncompressed)
}

fn calculate_checksum(input: &[ID]) -> u64 {
    let mut sum = 0;
    let mut last_i = input.len() - 1;
    'outer: for (i, id) in input.iter().enumerate()
    {
        if i > last_i {
            break;
        }

        let sum_id = match id.actual_id() {
            Some(id) => id,
            None => loop {
                let id = &input[last_i];
                match id.actual_id() {
                    Some(_id) => {
                        last_i -= 1;
                        break _id;
                    }
                    None => last_i -= 1,
                }

                if last_i <= i {
                    break 'outer;
                }
            },
        };
        let add_to_sum = i as u64 * sum_id;
        sum += add_to_sum;
        // println!("{} * {} = {} => {}", i, sum_id, add_to_sum, sum);
    }
    sum
}

fn get_uncompressed_layout(input: impl Iterator<Item = Block>) -> Vec<ID> {
    let mut bytes = vec![];
    for block in input {
        for _ in 0..block.length {
            bytes.push(block.id);
        }
    }
    bytes
}

fn get_file_layout(input: &str) -> impl Iterator<Item = Block> + '_ {
    let mut is_file = true;
    let mut cur_id = 0u64;
    input.bytes()
        .map(move |b| {
            // println!("{} =", char::from_u32(b as u32).unwrap());
            // println!("{}", b - b'0');
            let length = b - b'0';
            let id = if is_file {
                cur_id += 1;
                ID(cur_id)
            } else {
                ID::EMPTY
            };
            is_file = !is_file;
            Block { id, length }
        })
}


