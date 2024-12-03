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
    let mut instructions = vec![];
    instructions.extend(mul_instructions);
    instructions.extend(do_instructions);
    instructions.extend(dont_instructions);
    instructions.sort_by(|a, b| a.index.cmp(&b.index));

    let mut is_mul_enabled = true;
    let mut sum = 0;
    for info in instructions {
        match info.instruction {
            Instruction::Do => is_mul_enabled = true,
            Instruction::Dont => is_mul_enabled = false,
            Instruction::Mul(x, y) => {
                if is_mul_enabled {
                    sum += x * y
                }
            }
        }
    }

    sum
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
