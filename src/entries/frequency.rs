use std::collections::HashMap;
use std::env;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

pub(super) struct Frequencies(HashMap<String, EntryFrequency>);

impl Frequencies {
    pub fn load() -> Self {
        Self(match std::fs::read_to_string(Self::get_path()) {
            Ok(file) => toml::from_str(&file).expect("Failed to parse frequency history"),
            Err(_) => HashMap::new(),
        })
    }

    pub async fn save(&self) {
        let content = toml::to_string(&self.0).expect("Failed to serialize frequency history");
        let path = Self::get_path();
        if let Err(err) = smol::fs::write(&path, content).await {
            eprintln!(
                "Failed to save frequency history at {}: {}",
                path.to_string_lossy(),
                err
            );
        }
    }

    fn get_path() -> PathBuf {
        match env::var("XDG_CACHE_HOME") {
            Ok(config) => PathBuf::from(config).join("waystart.toml"),
            Err(_) => env::home_dir().unwrap().join(".cache/waystart.toml"),
        }
    }
}

impl Deref for Frequencies {
    type Target = HashMap<String, EntryFrequency>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Frequencies {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
