use std::time::Duration;

/// Represents time periods ranging from milliseconds to days
pub trait Eternity {
    fn humanize(&self) -> String;
    fn robotize(&self) -> String;
}

/// Represents time periods ranging from nanoseconds to milliseconds
pub trait NanoEternity {
    fn humanize(&self) -> String;
    fn robotize(&self) -> String;
}

/// Represents time periods ranging from milliseconds to minutes
pub trait ShortEternity {
    fn humanize(&self) -> String;
    fn robotize(&self) -> String;
}

/// Represents time periods ranging from seconds to hours
pub trait MediumEternity {
    fn humanize(&self) -> String;
    fn robotize(&self) -> String;
}


impl Eternity for Duration {
    fn humanize(&self) -> String {
        to_time_vec(self).into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<String>>()
            .join(" ").to_string()
    }

    fn robotize(&self) -> String {
        "bar".to_string()
    }
}

fn to_time_vec(dur: &Duration) -> Vec<Option<String>> {
    let units = WHOLE_UNITS;

    let mut accum = dur.as_secs();
    let mut res: Vec<Option<String>> = Vec::new();
    for unit in units.iter() {
        let t = accum / unit.interval;
        if t > 0 {
            accum -= t * unit.interval;
            res.push(Some(format!("{}{}", t, unit.suffix)));
        } else {
            res.push(None);
        }
    }
    match dur.subsec_millis() {
        0 => res.push(None),
        _ => res.push(Some(format!("{}ms", dur.subsec_millis())))
    }

    res
}

impl MediumEternity for Duration {
    fn humanize(&self) -> String {
        to_time_vec_hms(self).into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<String>>()
            .join(" ").to_string()
    }

    fn robotize(&self) -> String {
        "bar".to_string()
    }
}

fn to_time_vec_hms(dur: &Duration) -> Vec<Option<String>> {
    let units = &WHOLE_UNITS[1..4];

    let mut accum = dur.as_secs();
    let mut res: Vec<Option<String>> = Vec::new();
    for unit in units.iter() {
        let t = accum / unit.interval;
        if t > 0 {
            accum -= t * unit.interval;
            res.push(Some(format!("{}{}", t, unit.suffix)));
        } else {
            res.push(None);
        }
    }

    res
}

impl ShortEternity for Duration {
    fn humanize(&self) -> String {
        to_time_vec_msms(self).into_iter()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<String>>()
            .join(" ").to_string()
    }

    fn robotize(&self) -> String {
        to_time_vec_msms(self).into_iter()
            .map(unwrap_result)
            .collect::<Vec<String>>()
            .join(" ").to_string()
    }
}

fn to_time_vec_msms(dur: &Duration) -> Vec<Result<String, String>> {
    let units = &WHOLE_UNITS[2..4];

    let mut accum = dur.as_secs();
    let mut res: Vec<Result<String, String>> = Vec::new();
    for unit in units.iter() {
        let t = accum / unit.interval;
        if t > 0 {
            accum -= t * unit.interval;
            res.push(Ok(format!("{}{}", t, unit.suffix)));
        } else {
            res.push(Err(format!("0{}", unit.suffix)));
        }
    }
    match dur.subsec_millis() {
        0 => res.push(Err("0ms".to_string())),
        _ => res.push(Ok(format!("{}ms", dur.subsec_millis())))
    }

    res
}

impl NanoEternity for Duration {
    fn humanize(&self) -> String {
        to_time_vec_msusns(self).into_iter()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<String>>()
            .join(" ").to_string()
    }

    fn robotize(&self) -> String {
        to_time_vec_msusns(self).into_iter()
            .map(unwrap_result)
            .collect::<Vec<String>>()
            .join(" ").to_string()
    }
}

fn to_time_vec_msusns(dur: &Duration) -> Vec<Result<String, String>> {
    let frac_units = FRAC_UNITS;

    let mut accum = dur.subsec_nanos();
    let mut res: Vec<Result<String, String>> = Vec::new();
    for unit in frac_units.iter() {
        let t = accum / unit.interval;
        if t > 0 {
            accum -= t * unit.interval;
            res.push(Ok(format!("{}{}", t, unit.suffix)));
        } else {
            res.push(Err(format!("0{}", unit.suffix)));
        }
    }

    res
}

fn unwrap_result(x: Result<String, String>) -> String {
    match x {
        Ok(x) => x,
        Err(x) => x,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Unit<'a> {
    suffix: &'a str,
    interval: u64,
}

static WHOLE_UNITS: [Unit; 4] = [
    Unit { suffix: "d", interval: (24 * 3600) },
    Unit { suffix: "h", interval: 3600 },
    Unit { suffix: "m", interval: 60 },
    Unit { suffix: "s", interval: 1 },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct FracUnit<'a> {
    suffix: &'a str,
    interval: u32,
}

static FRAC_UNITS: [FracUnit; 3] = [
    FracUnit { suffix: "ms", interval: 1_000_000 },
    FracUnit { suffix: "us", interval: 1_000 },
    FracUnit { suffix: "ns", interval: 1 },
];

#[cfg(test)]
mod tests {

    mod eternity_tests {
        use std::time::Duration;
        use super::super::{Eternity, to_time_vec};

        #[test]
        fn secs() {
            let duration = Duration::from_secs(21);
            let exp = vec![None, None, None, Some("21s".to_string()), None];
            assert_eq!(exp, to_time_vec(&duration));
        }

        #[test]
        fn secs_human() {
            let duration = Duration::from_secs(21);
            let exp = "21s";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn mins() {
            let duration = Duration::from_secs(184);
            let exp = vec![None, None, Some("3m".to_string()), Some("4s".to_string()), None];
            assert_eq!(exp, to_time_vec(&duration));
        }

        #[test]
        fn mins_human() {
            let duration = Duration::from_secs(184);
            let exp = "3m 4s";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn hours() {
            let duration = Duration::from_secs(3672);
            let exp = vec![None, Some("1h".to_string()), Some("1m".to_string()), Some("12s".to_string()), None];
            assert_eq!(exp, to_time_vec(&duration));
        }

        #[test]
        fn hours_human() {
            let duration = Duration::from_secs(3672);
            let exp = "1h 1m 12s";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn days() {
            let duration = Duration::from_secs((24 * 3600) + 3672);
            let exp = vec![Some("1d".to_string()), Some("1h".to_string()), Some("1m".to_string()), Some("12s".to_string()), None];
            assert_eq!(exp, to_time_vec(&duration));
        }

        #[test]
        fn days_human() {
            let duration = Duration::from_secs((24 * 3600) + 3672);
            let exp = "1d 1h 1m 12s";
            assert_eq!(exp, &duration.humanize());
        }
    }

    mod medium_eternity_tests {
        use std::time::Duration;
        use super::super::{MediumEternity, to_time_vec_hms};

        #[test]
        fn secs() {
            let duration = Duration::from_secs(21);
            let exp = vec![None, None, Some("21s".to_string())];
            assert_eq!(exp, to_time_vec_hms(&duration));
        }

        #[test]
        fn secs_human() {
            let duration = Duration::from_secs(21);
            let exp = "21s";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn mins() {
            let duration = Duration::from_secs(184);
            let exp = vec![None, Some("3m".to_string()), Some("4s".to_string())];
            assert_eq!(exp, to_time_vec_hms(&duration));
        }

        #[test]
        fn mins_human() {
            let duration = Duration::from_secs(184);
            let exp = "3m 4s";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn hours() {
            let duration = Duration::from_secs(3672);
            let exp = vec![Some("1h".to_string()), Some("1m".to_string()), Some("12s".to_string())];
            assert_eq!(exp, to_time_vec_hms(&duration));
        }

        #[test]
        fn hours_human() {
            let duration = Duration::from_secs(3672);
            let exp = "1h 1m 12s";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn days() {
            let duration = Duration::from_secs((24 * 3600) + 3672);
            let exp = vec![Some("25h".to_string()), Some("1m".to_string()), Some("12s".to_string())];
            assert_eq!(exp, to_time_vec_hms(&duration));
        }

        #[test]
        fn days_human() {
            let duration = Duration::from_secs((24 * 3600) + 3672);
            let exp = "25h 1m 12s";
            assert_eq!(exp, &duration.humanize());
        }
    }

    mod short_eternity {
        use std::time::Duration;
        use super::super::{ShortEternity, to_time_vec_msms};

        #[test]
        fn millis() {
            let duration = Duration::from_millis(2134);
            let exp = vec![Err("0m".to_string()), Ok("2s".to_string()), Ok("134ms".to_string())];
            assert_eq!(exp, to_time_vec_msms(&duration));
        }

        #[test]
        fn millis_human() {
            let duration = Duration::from_millis(2134);
            let exp = "2s 134ms";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn millis_bot() {
            let duration = Duration::from_millis(2134);
            let exp = "0m 2s 134ms";
            assert_eq!(exp, &duration.robotize());
        }

        #[test]
        fn secs() {
            let duration = Duration::from_secs(21);
            let exp = vec![Err("0m".to_string()), Ok("21s".to_string()), Err("0ms".to_string())];
            assert_eq!(exp, to_time_vec_msms(&duration));
        }

        #[test]
        fn secs_human() {
            let duration = Duration::from_secs(21);
            let exp = "21s";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn secs_bot() {
            let duration = Duration::from_secs(21);
            let exp = "0m 21s 0ms";
            assert_eq!(exp, &duration.robotize());
        }

        #[test]
        fn mins() {
            let duration = Duration::from_secs(184);
            let exp = vec![Ok("3m".to_string()), Ok("4s".to_string()), Err("0ms".to_string())];
            assert_eq!(exp, to_time_vec_msms(&duration));
        }

        #[test]
        fn mins_human() {
            let duration = Duration::from_secs(184);
            let exp = "3m 4s";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn mins_bot() {
            let duration = Duration::from_secs(184);
            let exp = "3m 4s 0ms";
            assert_eq!(exp, &duration.robotize());
        }

        #[test]
        fn hours() {
            let duration = Duration::from_secs(3672);
            let exp = vec![Ok("61m".to_string()), Ok("12s".to_string()), Err("0ms".to_string())];
            assert_eq!(exp, to_time_vec_msms(&duration));
        }

        #[test]
        fn hours_human() {
            let duration = Duration::from_secs(3672);
            let exp = "61m 12s";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn hours_bot() {
            let duration = Duration::from_secs(3672);
            let exp = "61m 12s 0ms";
            assert_eq!(exp, &duration.robotize());
        }

        #[test]
        fn days() {
            let duration = Duration::from_secs((24 * 3600) + 3672);
            let exp = vec![Ok("1501m".to_string()), Ok("12s".to_string()), Err("0ms".to_string())];
            assert_eq!(exp, to_time_vec_msms(&duration));
        }

        #[test]
        fn days_human() {
            let duration = Duration::from_secs((24 * 3600) + 3672);
            let exp = "1501m 12s";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn days_bot() {
            let duration = Duration::from_secs((24 * 3600) + 3672);
            let exp = "1501m 12s 0ms";
            assert_eq!(exp, &duration.robotize());
        }
    }

    mod nano_eternity {
        use std::time::Duration;
        use super::super::{NanoEternity, to_time_vec_msusns};

        #[test]
        fn nanos() {
            let duration = Duration::from_nanos(2134);
            let exp = vec![Err("0ms".to_string()), Ok("2us".to_string()), Ok("134ns".to_string())];
            assert_eq!(exp, to_time_vec_msusns(&duration));
        }

        #[test]
        fn nanos_human() {
            let duration = Duration::from_nanos(2134);
            let exp = "2us 134ns";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn nanos_bot() {
            let duration = Duration::from_nanos(2134);
            let exp = "0ms 2us 134ns";
            assert_eq!(exp, &duration.robotize());
        }

        #[test]
        fn micros() {
            let duration = Duration::from_nanos(20_134);
            let exp = vec![Err("0ms".to_string()), Ok("20us".to_string()), Ok("134ns".to_string())];
            assert_eq!(exp, to_time_vec_msusns(&duration));
        }

        #[test]
        fn micros_human() {
            let duration = Duration::from_nanos(20_134);
            let exp = "20us 134ns";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn micros_bot() {
            let duration = Duration::from_nanos(20_134);
            let exp = "0ms 20us 134ns";
            assert_eq!(exp, &duration.robotize());
        }

        #[test]
        fn millis() {
            let duration = Duration::from_nanos(2_134_567);
            let exp = vec![Ok("2ms".to_string()), Ok("134us".to_string()), Ok("567ns".to_string())];
            assert_eq!(exp, to_time_vec_msusns(&duration));
        }

        #[test]
        fn millis_human() {
            let duration = Duration::from_nanos(2_134_567);
            let exp = "2ms 134us 567ns";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn millis_bot() {
            let duration = Duration::from_nanos(2_134_567);
            let exp = "2ms 134us 567ns";
            assert_eq!(exp, &duration.robotize());
        }

        #[test]
        fn secs_are_dropped() {
            let duration = Duration::from_nanos(2_134_567_789);
            let exp = vec![Ok("134ms".to_string()), Ok("567us".to_string()), Ok("789ns".to_string())];
            assert_eq!(exp, to_time_vec_msusns(&duration));
        }

        #[test]
        fn secs_are_dropped_human() {
            let duration = Duration::from_nanos(2_134_567_789);
            let exp = "134ms 567us 789ns";
            assert_eq!(exp, &duration.humanize());
        }

        #[test]
        fn secs_are_dropped_bot() {
            let duration = Duration::from_nanos(2_134_567_789);
            let exp = "134ms 567us 789ns";
            assert_eq!(exp, &duration.robotize());
        }
    }
}
