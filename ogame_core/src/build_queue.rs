use serde::{Deserialize, Serialize};

use crate::{build_time_trait::BuildTime, error::Result};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildQueueItem<T> {
    pub r#type: T,
    pub finish_date: usize,
}

impl<T> BuildQueueItem<T>
where
    T: BuildTime,
{
    pub fn new(r#type: T, finish_date: usize) -> Self {
        BuildQueueItem {
            r#type,
            finish_date,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BuildQueue<T>
where
    T: BuildTime,
{
    pub items: Vec<BuildQueueItem<T>>,
}

impl<T> BuildQueue<T>
where
    T: BuildTime + Clone,
{
    pub fn new(items: Vec<BuildQueueItem<T>>) -> Self {
        BuildQueue { items }
    }

    pub fn push(&mut self, item: T, level: usize) {
        let finish_date = self.items.last().map(|item| item.finish_date).unwrap_or(
            web_time::SystemTime::now()
                .duration_since(web_time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
        ) + item.build_time(level);

        self.items.push(BuildQueueItem {
            r#type: item,
            finish_date,
        });
    }

    pub fn tick(&mut self, now: usize) -> Result<Vec<T>> {
        let solved = self.get_solved_elements(now);
        self.items = self.items.drain(solved.len()..).collect();

        Ok(solved)
    }

    pub fn calc_tick_until_first_completion(&self, now: usize) -> usize {
        let first_finish_date = self
            .items
            .first()
            .map(|item| item.finish_date)
            .unwrap_or(now);

        if first_finish_date < now {
            first_finish_date
        } else {
            now
        }
    }

    pub fn get_solved_elements(&self, now: usize) -> Vec<T> {
        self.items
            .iter()
            .filter(|item| item.finish_date <= now)
            .map(|item| item.r#type.clone())
            .collect()
    }
}
