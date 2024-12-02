pub fn solution(input: String) -> u32 {
    let safe_count = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|report| {
            let mut is_safe = false;
            let is_increasing = (report[0] as i32 - report[1] as i32) > 0;
            for i in 0..report.len() - 1 {
                let a = report[i];
                let b = report[i + 1];
                let abs_diff = a.abs_diff(b);
                if abs_diff > 0 && abs_diff < 4 {
                    let diff = a as i32 - b as i32;

                    if (is_increasing && diff > 0) || (!is_increasing && diff < 0) {
                        is_safe = true;
                    } else {
                        is_safe = false;
                        break;
                    }
                } else {
                    is_safe = false;
                    break;
                }
            }
            is_safe
        })
        .count();
    safe_count as u32
}
