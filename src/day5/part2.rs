pub fn solution(input: String) -> u32 {
    let lines = input.lines();

    let mut rules = vec![];
    let mut updates = vec![];

    let mut is_rules = true;
    for line in lines {
        if is_rules {
            if line == "" {
                is_rules = false;
            } else {
                let mut ints = line.split("|").map(|str| str.parse::<u32>().unwrap());

                let left = ints.next().unwrap();
                let right = ints.next().unwrap();
                rules.push((left, right));
            }
        } else {
            updates.push(
                line.split(",")
                    .map(|str| str.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
    }

    rules.sort_by(|a, b| a.1.cmp(&b.1));

    updates
        .iter_mut()
        .filter_map(|update| {
            let mut updates_in_correct_order = true;
            for (left, right) in &rules {
                if let Some(pos_left) = update.iter().position(|x| *x == *left) {
                    if let Some(pos_right) = update.iter().position(|x| *x == *right) {
                        if pos_left > pos_right {
                            updates_in_correct_order = false;
                            if pos_left > pos_right {
                                update.remove(pos_right);

                                let new_index = pos_left;
                                if new_index >= update.len() {
                                    update.push(*right);
                                } else {
                                    update.insert(new_index, *right);
                                }
                            }
                        }
                    }
                }
            }

            if updates_in_correct_order {
                None
            } else {
                Some(update[(update.len() as f32 / 2.0) as usize])
            }
        })
        .sum()
}
