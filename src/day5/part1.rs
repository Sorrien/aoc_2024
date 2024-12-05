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

    updates
        .iter()
        .map(|update| {
            let mut updates_in_correct_order = true;
            for (left, right) in &rules {
                if let Some(pos_left) = update.iter().position(|x| *x == *left) {
                    if let Some(pos_right) = update.iter().position(|x| *x == *right) {
                        if pos_left > pos_right {
                            updates_in_correct_order = false;
                            break;
                        }
                    }
                }
            }

            if updates_in_correct_order {
                update[(update.len() as f32 / 2.0) as usize]
            } else {
                0
            }
        })
        .sum()
}
