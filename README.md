# Clock Tower

A simple, thread-safe Rust library for measuring the execution time of code blocks and expressions. Measurements are stored in a global registry for easy retrieval.

## License

This project is licensed under the **GPL-3.0** license. This ensures that any software using this library must also be open source.

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
clock_tower = "0.1.0" 
```

## Usage

### Measuring Code

Use the `measure!` macro to wrap expressions or blocks. It returns the value of the expression while recording the time it took.

```rust
use clock_tower::{measure, time_registry};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Measure a simple expression
    let result = measure!("simple_math", 21 + 21);
    println!("Result: {}", result);

    // Measure a block of code
    let complex_result = measure!("heavy_task", {
        sleep(Duration::from_millis(100));
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        sum
    });
    println!("Complex Result: {}", complex_result);
}
```


### Retrieving Metrics

You can access the recorded timings using the `time_registry` module.

```rust
use clock_tower::time_registry;

fn print_stats() {
    // Get a specific measurement by name
    if let Some((start, end, duration)) = time_registry::get("heavy_task") {
        println!("Task 'heavy_task' took {:?}", duration);
    }

    // Iterate over all measurements
    println!("All measurements:");
    for (name, (_start, _end, duration)) in time_registry::all() {
        println!("{} -> {:?}", name, duration);
    }
    
    // Clear registry if needed to start fresh
    time_registry::clear();
}
```