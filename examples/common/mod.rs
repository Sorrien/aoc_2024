use std::time::Instant;

pub fn run_solution<F: Fn() -> String>(measure: bool, f: F) {
    if measure {
        let mut timings = vec![];
        for _ in 0..10000 {
            let instant = Instant::now();
            f();

            timings.push((Instant::now() - instant).as_nanos());
        }
        println!(
            "avg time in nanos: {}",
            timings.iter().sum::<u128>() as f64 / timings.iter().count() as f64
        );
    } else {
        let string = f();
        println!("{}", string);
    }
}
