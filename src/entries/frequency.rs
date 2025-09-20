use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::LazyLock;
use std::time::SystemTime;

use gpui::SharedString;
use serde::{Deserialize, Serialize};

pub(super) struct Frequencies(HashMap<SharedString, EntryFrequency>);

impl Frequencies {
    pub fn load() -> Self {
        Self(match std::fs::read_to_string(&*FREQUENCIES_SAVE_PATH) {
            Ok(file) => toml::from_str(&file).expect("Failed to parse frequency history"),
            Err(_) => HashMap::new(),
        })
    }

    pub async fn save(&self) {
        let content = toml::to_string(&self.0).expect("Failed to serialize frequency history");
        if let Err(err) = smol::fs::write(&*FREQUENCIES_SAVE_PATH, content).await {
            eprintln!(
                "Failed to save frequency history at {}: {}",
                FREQUENCIES_SAVE_PATH.to_string_lossy(),
                err
            );
        }
    }
}

impl Deref for Frequencies {
    type Target = HashMap<SharedString, EntryFrequency>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Frequencies {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Serialize, Deserialize, Eq)]
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

impl Default for EntryFrequency {
    fn default() -> Self {
        Self {
            score: 0,
            last_used: SystemTime::UNIX_EPOCH,
        }
    }
}

impl Ord for EntryFrequency {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score().cmp(&other.score()) {
            Ordering::Equal => self.last_used.cmp(&other.last_used),
            order => order,
        }
    }
}

impl PartialOrd for EntryFrequency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EntryFrequency {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

static FREQUENCIES_SAVE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    dirs::cache_dir()
        .expect("Failed to get cache directory")
        .join("waystart.toml")
});
