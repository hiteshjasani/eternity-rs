use std::time::Duration;

pub trait Eternity {
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
    let mut accum = dur.as_secs();
    let mut res: Vec<Option<String>> = Vec::new();
    for unit in UNITS.iter() {
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

static UNITS: [Unit; 4] = [
    Unit { suffix: "d", interval: (24 * 3600) },
    Unit { suffix: "h", interval: 3600 },
    Unit { suffix: "m", interval: 60 },
    Unit { suffix: "s", interval: 1 },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Unit<'a> {
    suffix: &'a str,
    interval: u64,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

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
