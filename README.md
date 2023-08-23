# eternity-rs
Formatting human and bot readable time durations


## Motivation

Rust does not implement `Display` for `Duration`. Even if it did, usage would vary between wanting things easy to read versus having them readable by automated tools.  This crate tries to hit the major use cases.

## Simple Example

```
use std::time::Duration;
use eternity-rs::Eternity;

let duration = Duration::from_secs(3672);
println!("{}", &duration.humanize());
=> 1h 1m 12s

let duration = Duration::from_secs(3622);
println!("{}", &duration.humanize());
=> 1h 22s

let duration = Duration::from_secs(127);
println!("{}", &duration.humanize());
=> 2m 7s
```
