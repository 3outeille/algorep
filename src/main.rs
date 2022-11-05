use serde::{Deserialize, Serialize};

use std::sync::mpsc::{self, RecvError};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
enum ConfigTime {
    Second(u64),
    Millisecond(u64),
    Microsecond(u64),
    Nanosecond(u64),
    Minute(u64),
}

impl ConfigTime {
    #[allow(dead_code)]
    fn to_duration(self) -> Duration {
        match self {
            Self::Second(n) => Duration::from_secs(n),
            Self::Millisecond(n) => Duration::from_millis(n),
            Self::Microsecond(n) => Duration::from_micros(n),
            Self::Nanosecond(n) => Duration::from_nanos(n),
            Self::Minute(n) => Duration::from_secs(n * 60),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    servers: usize,
    election_timeout: (ConfigTime, ConfigTime),
}

const CONFIG_STR: &'static str = include_str!("../config/config.ron");

fn main() {
    let config: Config = ron::from_str(CONFIG_STR).expect("");
}
