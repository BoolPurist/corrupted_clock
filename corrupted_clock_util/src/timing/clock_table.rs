use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{CountDown, Stopwatch, UtcTimeImpl};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ClockTable<T = UtcTimeImpl>
where
    T: Default,
{
    stopwatches: HashMap<String, Stopwatch<T>>,
    count_downs: HashMap<String, CountDown<T>>,
}

#[derive(Debug, Error)]
#[error("Key {0} is already taken")]
pub struct AlreadyAdded(String);

impl<T> ClockTable<T>
where
    T: Default,
{
    pub fn new(
        stopwatches: HashMap<String, Stopwatch<T>>,
        count_downs: HashMap<String, CountDown<T>>,
    ) -> Self {
        Self {
            stopwatches,
            count_downs,
        }
    }

    pub fn all_stopwatches(&self) -> impl Iterator<Item = (&str, &Stopwatch<T>)> {
        self.stopwatches
            .iter()
            .map(|(key, value)| (key.as_str(), value))
    }

    pub fn all_count_downs(&self) -> impl Iterator<Item = (&str, &CountDown<T>)> {
        self.count_downs
            .iter()
            .map(|(key, value)| (key.as_str(), value))
    }

    pub fn mut_all_count_downs(&mut self) -> impl Iterator<Item = &mut CountDown<T>> {
        self.count_downs.values_mut().into_iter()
    }

    pub fn mut_all_stop_watches(&mut self) -> impl Iterator<Item = &mut Stopwatch<T>> {
        self.stopwatches.values_mut().into_iter()
    }

    pub fn remove_count_down(&mut self, key: &str) -> bool {
        self.count_downs.remove(key).is_some()
    }

    pub fn remove_stopwatch(&mut self, key: &str) -> bool {
        self.stopwatches.remove(key).is_some()
    }

    pub fn remove_all_stopwatches(&mut self) {
        self.stopwatches = Default::default();
    }

    pub fn remove_all_count_donws(&mut self) {
        self.count_downs = Default::default();
    }

    pub fn has_stop_watch(&self, key: &str) -> bool {
        self.stopwatches.contains_key(key)
    }

    pub fn has_count_down(&self, key: &str) -> bool {
        self.count_downs.contains_key(key)
    }

    pub fn modify_stopwatch(&mut self, key: &str) -> Option<&mut Stopwatch<T>> {
        self.stopwatches.get_mut(key)
    }

    pub fn modify_count_down(&mut self, key: &str) -> Option<&mut CountDown<T>> {
        self.count_downs.get_mut(key)
    }

    pub fn mut_stopwatch(&mut self, key: &str) -> Option<&mut Stopwatch<T>> {
        self.stopwatches.get_mut(key)
    }

    pub fn mut_count_down(&mut self, key: &str) -> Option<&mut CountDown<T>> {
        self.count_downs.get_mut(key)
    }

    pub fn get_stopwatch(&self, key: &str) -> Option<&Stopwatch<T>> {
        self.stopwatches.get(key)
    }

    pub fn get_count_down(&self, key: &str) -> Option<&CountDown<T>> {
        self.count_downs.get(key)
    }

    pub fn add_stopwatch(&mut self, key: String, sw: Stopwatch<T>) -> Result<(), AlreadyAdded> {
        if self.stopwatches.contains_key(&key) {
            return Err(AlreadyAdded(key));
        }
        _ = self.stopwatches.insert(key, sw);
        Ok(())
    }

    pub fn add_count_down(&mut self, key: String, sw: CountDown<T>) -> Result<(), AlreadyAdded> {
        if self.count_downs.contains_key(&key) {
            return Err(AlreadyAdded(key));
        }
        _ = self.count_downs.insert(key, sw);
        Ok(())
    }
}
