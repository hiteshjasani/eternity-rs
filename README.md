# eternity-rs

Answering the question, "how long did that take?", formattable to be readable by humans and bots.

## Motivation

Rust does not implement `Display` for `std::time::Duration`. But even if it did, there wouldn't be a format that would be usable across the board.  Formats that are human readable are hard to parse for bots, and vice versa.  This crate provides extension traits to enable formatting durations for most cases.

## Simple Example

```
use std::time::Duration;
use eternity_rs::Eternity;

let duration = Duration::from_secs(3672);
println!("{}", &duration.humanize());
=> 1h 1m 12s
println!("{}", &duration.robotize());
=> 0d 1h 1m 12s 0ms

let duration = Duration::from_secs(3622);
println!("{}", &duration.humanize());
=> 1h 22s
println!("{}", &duration.robotize());
=> 0d 1h 0m 22s 0ms

let duration = Duration::from_secs(127);
println!("{}", &duration.humanize());
=> 2m 7s
println!("{}", &duration.robotize());
=> 0d 0h 2m 7s 0ms
```

## Tests

`cargo test`

## Benchmarking

`cargo bench`
