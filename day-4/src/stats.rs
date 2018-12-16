use std::collections::HashMap;

use chrono::Timelike;

use crate::log::{LogAction, LogEntry};

type Id = u32;

#[derive(Debug, PartialEq, Clone)]
pub struct GuardIndividualStats {
    pub id: Id,
    total: u32,
    per_minute: Vec<u32>,
}

impl GuardIndividualStats {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            total: 0,
            per_minute: vec![0; 60],
        }
    }

    pub fn log_sleep(&mut self, start: u32, end: u32) {
        self.total += end - start;

        for minute in start..end {
            self.per_minute[minute as usize] += 1;
        }
    }

    pub fn worst_minute(&self) -> (usize, &u32) {
        self.per_minute
            .iter()
            .enumerate()
            .max_by_key(|(_, &stat)| stat)
            .unwrap()
    }
}

pub struct GuardStats {
    pub data: HashMap<Id, GuardIndividualStats>,
}

#[derive(Copy, Clone)]
pub enum WorstSelectionStrategy {
    Frequency,
    Total,
}

impl GuardStats {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn process_log<'a>(&mut self, log: impl Iterator<Item = &'a LogEntry>) {
        let mut current_stat: Option<GuardIndividualStats> = None;
        let mut current_sleep_start = None;

        for entry in log {
            match entry.action {
                LogAction::BeginsShift(id) => {
                    if let Some(stat) = current_stat.take() {
                        self.data.insert(stat.id, stat);
                    }
                    current_stat = self
                        .data
                        .remove(&id)
                        .or_else(|| Some(GuardIndividualStats::new(id)));
                }

                LogAction::FallsAsleep => {
                    debug_assert_eq!(entry.datetime.hour(), 0);
                    debug_assert_eq!(current_sleep_start, None);

                    current_sleep_start = Some(entry.datetime.minute());
                }

                LogAction::WakesUp => {
                    let end = entry.datetime.minute();
                    let start = current_sleep_start.take().unwrap();

                    debug_assert!(current_stat.is_some());

                    if let Some(stat) = current_stat.as_mut() {
                        stat.log_sleep(start, end);
                    }
                }
            };
        }

        if let Some(stat) = current_stat.take() {
            self.data.insert(stat.id, stat);
        }

        debug_assert!(current_sleep_start.is_none());
    }

    pub fn get_worst_guard(
        &self,
        strategy: WorstSelectionStrategy,
    ) -> Option<&GuardIndividualStats> {
        match strategy {
            WorstSelectionStrategy::Total => self.data.values().max_by_key(|stat| stat.total),
            WorstSelectionStrategy::Frequency => {
                self.data.values().max_by_key(|stat| stat.worst_minute().1)
            }
        }
    }
}
