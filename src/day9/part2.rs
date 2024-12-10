use std::usize;

pub fn solution(input: String) -> u64 {
    let mut file_id = 0;
    let blocks = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .enumerate()
        .map(|(i, x)| {
            let is_free_space = i % 2 != 0;
            let id = if is_free_space { usize::MAX } else { file_id };
            if !is_free_space {
                file_id += 1;
            }

            if is_free_space {
                Block::new_free(x)
            } else {
                Block::new_file(id, x)
            }
        })
        .collect::<Vec<_>>();

    let mut sorted_blocks = blocks
        .iter()
        .map(|block| vec![block.clone()])
        .collect::<Vec<_>>();

    for i in (0..blocks.len()).rev() {
        let file_block = &blocks[i];

        if !file_block.is_free {
            for j in 0..blocks.len() {
                if j > i {
                    break;
                }
                let block_slot = sorted_blocks[j].clone();
                let mut is_swap_made = false;

                for k in 0..block_slot.len() {
                    let block = block_slot[k].clone();

                    if block.is_free {
                        if block.length == file_block.length {
                            sorted_blocks[j][k] = file_block.clone();
                            sorted_blocks[i][0] = block.clone();
                            is_swap_made = true;
                            break;
                        } else if block.length > file_block.length {
                            sorted_blocks[j][k] = file_block.clone();
                            sorted_blocks[j]
                                .insert(k + 1, Block::new_free(block.length - file_block.length));
                            sorted_blocks[i][0] = Block::new_free(file_block.length);
                            is_swap_made = true;
                            break;
                        }
                    }
                }

                if is_swap_made {
                    break;
                }
            }
        }
    }

    sorted_blocks
        .iter()
        .flatten()
        .filter_map(|block| {
            if block.is_free {
                Some(vec![0; block.length as usize])
            } else {
                Some(vec![block.id; block.length as usize])
            }
        })
        .flatten()
        .enumerate()
        .map(|(i, block)| i as u64 * block as u64)
        .sum()
}

#[derive(Clone)]
pub struct Block {
    pub id: usize,
    pub length: u8,
    pub is_free: bool,
}

impl Block {
    pub fn new_file(id: usize, length: u8) -> Self {
        Self {
            id,
            length,
            is_free: false,
        }
    }

    pub fn new_free(length: u8) -> Self {
        Self {
            id: usize::MAX,
            length,
            is_free: true,
        }
    }
}
