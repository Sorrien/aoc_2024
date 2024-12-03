use regex::Regex;

pub fn solution(input: String) -> u32 {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("bad regex!");

    let do_regex = Regex::new(r"do\(\)").expect("bad regex!");
    let dont_regex = Regex::new(r"don't\(\)").expect("bad regex!");

    let mul_instructions = mul_regex
        .captures_iter(&input)
        .map(|result| {
            let start_index = result.get(0).unwrap().start();
            let (_, [left, right]) = result.extract();

            (start_index, [left, right])
        })
        .map(|(index, items)| {
            let items = items.map(|item| item.parse::<u32>().unwrap());
            InstructionInfo::new(Instruction::Mul(items[0], items[1]), index)
        });

    let do_instructions = do_regex.captures_iter(&input).map(|result| {
        let start_index = result.get(0).unwrap().start();

        InstructionInfo::new(Instruction::Do, start_index)
    });

    let dont_instructions = dont_regex.captures_iter(&input).map(|result| {
        let start_index = result.get(0).unwrap().start();

        InstructionInfo::new(Instruction::Dont, start_index)
    });

    let mut is_mul_enabled = true;
    mul_instructions
        .chain(do_instructions)
        .chain(dont_instructions)
        .filter_map(|info| match info.instruction {
            Instruction::Do => {
                is_mul_enabled = true;
                None
            }
            Instruction::Dont => {
                is_mul_enabled = false;
                None
            }
            Instruction::Mul(x, y) => {
                if is_mul_enabled {
                    Some(x * y)
                } else {
                    None
                }
            }
        })
        .sum()
}

pub enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

pub struct InstructionInfo {
    pub instruction: Instruction,
    pub index: usize,
}

impl InstructionInfo {
    pub fn new(instruction: Instruction, index: usize) -> Self {
        Self { instruction, index }
    }
}
