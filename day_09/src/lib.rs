use std::char;


#[derive(Debug, Clone, Copy)]
struct ID(u64);

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

#[derive(Debug, Clone)]
struct Block {
    id: ID,
    length: u8,
}

impl Block {
    fn is_free_space(&self) -> bool {
        self.id.is_free_space()
    }
}

struct BlockLayout {
    blocks: Vec<Block>,
}

impl BlockLayout {
    fn from_num_layout(s: &str) -> Self {
        let blocks = get_file_layout(s.trim()).collect();
        Self { blocks }
    }

    fn reorganize_method_2(&mut self) {
        self.trim_end();
        let max_id = self.blocks.iter()
            .rev()
            .flat_map(|b| b.id.actual_id())
            .next()
            .unwrap();

        // self.debug_display();
        for id_to_move in (1..=max_id).rev() {
            let (mut block_i, required_length) = self.find_index_and_length(id_to_move).unwrap();
            // println!("{}: Requires {} !!!! {:?}", id_to_move, required_length, self.blocks[block_i].id.actual_id());
            let space_to_place_search = self.blocks[..block_i]
                .iter_mut()
                .enumerate()
                .find(|(_, b)| b.is_free_space() && b.length >= required_length);
            let (free_i, space_to_place) = match space_to_place_search {
                Some(b) => b,
                None => continue,
            };
            // println!("Found space at index {} with space {}", free_i, space_to_place.length);

            space_to_place.length -= required_length;
            if space_to_place.length == 0 {
                self.blocks.remove(free_i);
                block_i -= 1;
            }

            let Block { id, length } = self.blocks[block_i];
            self.blocks[block_i].id = ID::EMPTY;
            self.blocks.insert(free_i, Block { id, length });
            self.trim_end();
            // self.debug_display();
        }
    }

    fn find_index_and_length(&self, id: u64) -> Option<(usize, u8)> {
        self.blocks.iter()
            .enumerate()
            .find(|(_, b)| b.id.actual_id().is_some_and(|block_id| block_id == id))
            .map(|(i, b)| (i, b.length))
    }

    fn trim_end(&mut self) {
        while self.blocks.last().is_some_and(|b| b.is_free_space()) {
            self.blocks.pop();
        }
    }

    fn checksum(&self) -> u64 {
        let mut sum = 0;
        let mut i = 0u64;
        for block in &self.blocks {
            match block.id.actual_id() {
                Some(id) => for _ in 0..block.length {
                    sum += id * i;
                    i += 1;
                },
                None => i += block.length as u64,
            }
        }
        sum
    }

    fn debug_display(&self) {
        let mut s = String::new();
        for block in &self.blocks {
            let ch = match block.id.actual_id() {
                Some(id) => char::from_u32(id as u32 + b'0' as u32).unwrap(),
                None => '.',
            };
            for _ in 0..block.length {
                s.push(ch);
            }
        }
        println!("{}", s);
    }
}

pub fn get_disk_checksum(input: &str) -> u64 {
    let layout_iter = get_file_layout(input.trim());
    // println!("{:?}", layout);
    let uncompressed = get_uncompressed_layout(layout_iter);
    // println!("Uncompressed has length {}", uncompressed.len());
    calculate_checksum(&uncompressed)
}

pub fn get_disk_checksum_method_2(input: &str) -> u64 {
    let mut layout = BlockLayout::from_num_layout(input);
    layout.reorganize_method_2();
    // layout.debug_display();
    layout.checksum()
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


