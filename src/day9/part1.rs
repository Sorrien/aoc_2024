pub fn solution(input: String) -> u64 {
    let mut file_id = 0;
    let blocks = input
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .enumerate()
        .map(|(i, length)| {
            if i % 2 == 0 {
                let id = file_id;
                file_id += 1;
                vec![Some(id); length as usize]
            } else {
                vec![None; length as usize]
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut sorted_blocks = Vec::<Option<usize>>::with_capacity(blocks.len());

    let mut rev_index = blocks.len() - 1_usize;
    for i in 0..blocks.len() {
        if i > rev_index {
            break;
        }
        let x = if let Some(id) = blocks[i] {
            Some(id)
        } else {
            let mut x = None;
            loop {
                let block = blocks[rev_index];
                if block.is_some() {
                    x = block;
                    rev_index -= 1;
                    break;
                } else {
                    rev_index -= 1;
                    if i == rev_index {
                        break;
                    }
                }
            }
            x
        };

        sorted_blocks.push(x);
    }

    sorted_blocks
        .iter()
        .enumerate()
        .filter_map(|(i, block)| {
            if let Some(block) = block {
                Some((i * block) as u64)
            } else {
                None
            }
        })
        .sum()
}
