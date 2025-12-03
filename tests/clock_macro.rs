use std::time::Duration;

use clock_tower::{measure, time_registry::get};

#[test]
fn records_simple_expression() {
    let result = measure!("simple", 21 + 21);

    assert_eq!(result, 42);

    let elapsed = log_measurement("simple");
    assert!(elapsed >= Duration::ZERO);
}

#[test]
fn records_block_expression() {
    let result = measure!("block", {
        let mut temp = 0;
        for i in 0..3 {
            temp += i;
        }
        temp
    });

    assert_eq!(result, 3);

    let elapsed = log_measurement("block");
    assert!(elapsed >= Duration::ZERO);
}

fn log_measurement(name: &str) -> Duration {
    let (start, end, elapsed) = get(name).expect("measurement should exist");
    println!("{} -> start={:?}, end={:?}, elapsed={:?}", name, start, end, elapsed);
    elapsed
}
