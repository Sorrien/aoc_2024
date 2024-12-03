use regex::Regex;

pub fn solution(input: String) -> u32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("bad regex!");
    regex
        .captures_iter(&input)
        .map(|result| {
            let (_, [left, right]) = result.extract();

            [left, right]
        })
        .map(|items| {
            let items = items.map(|item| item.parse::<u32>().unwrap());
            items[0] * items[1]
        })
        .sum()
}
