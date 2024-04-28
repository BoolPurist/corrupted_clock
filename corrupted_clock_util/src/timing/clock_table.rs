use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{CountDown, Stopwatch};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ClockTable {
    stopwatches: HashMap<String, Stopwatch>,
    count_downs: HashMap<String, CountDown>,
}

#[derive(Debug, Error)]
#[error("Key {0} is already taken")]
pub struct AlreadyAdded(String);

impl ClockTable {
    pub fn all_stopwatches(&self) -> impl Iterator<Item = (&str, &Stopwatch)> {
        self.stopwatches
            .iter()
            .map(|(key, value)| (key.as_str(), value))
    }

    pub fn all_count_downs(&self) -> impl Iterator<Item = (&str, &CountDown)> {
        self.count_downs
            .iter()
            .map(|(key, value)| (key.as_str(), value))
    }

    pub fn remove_count_down(&mut self, key: &str) -> bool {
        self.count_downs.remove(key).is_some()
    }

    pub fn has_stop_watch(&self, key: &str) -> bool {
        self.stopwatches.contains_key(key)
    }

    pub fn has_count_down(&self, key: &str) -> bool {
        self.count_downs.contains_key(key)
    }

    pub fn remove_stopwatch(&mut self, key: &str) -> bool {
        self.stopwatches.remove(key).is_some()
    }

    pub fn modify_stopwatch(&mut self, key: &str) -> Option<&mut Stopwatch> {
        self.stopwatches.get_mut(key)
    }

    pub fn modify_count_down(&mut self, key: &str) -> Option<&mut CountDown> {
        self.count_downs.get_mut(key)
    }

    pub fn get_stopwatch(&self, key: &str) -> Option<&Stopwatch> {
        self.stopwatches.get(key)
    }

    pub fn get_count_down(&self, key: &str) -> Option<&CountDown> {
        self.count_downs.get(key)
    }

    pub fn add_stopwatch(&mut self, key: String, sw: Stopwatch) -> Result<(), AlreadyAdded> {
        if self.stopwatches.contains_key(&key) {
            return Err(AlreadyAdded(key));
        }
        _ = self.stopwatches.insert(key, sw);
        Ok(())
    }

    pub fn add_count_down(&mut self, key: String, sw: CountDown) -> Result<(), AlreadyAdded> {
        if self.count_downs.contains_key(&key) {
            return Err(AlreadyAdded(key));
        }
        _ = self.count_downs.insert(key, sw);
        Ok(())
    }
}
