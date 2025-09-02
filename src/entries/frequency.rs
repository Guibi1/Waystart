use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct EntryFrequency {
    score: u32,
    last_used: SystemTime,
}

impl EntryFrequency {
    pub fn new() -> Self {
        Self {
            score: 1,
            last_used: SystemTime::now(),
        }
    }

    pub fn score(&self) -> u32 {
        let Ok(time_passed) = self.last_used.elapsed() else {
            return 0;
        };

        if time_passed.as_secs() < 60 * 60 {
            // One hour
            self.score * 4
        } else if time_passed.as_secs() < 60 * 60 * 24 {
            // One day
            self.score * 2
        } else if time_passed.as_secs() < 60 * 60 * 24 * 7 {
            // One week
            self.score / 2
        } else {
            self.score / 4
        }
    }

    pub fn increment(&mut self) {
        self.score += 1;
        self.last_used = SystemTime::now();
    }
}
