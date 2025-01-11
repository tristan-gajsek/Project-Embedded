use anyhow::{anyhow, Result};
use std::{collections::VecDeque, sync::Mutex};

use crate::data::NoiseData;

pub struct Graph {
    data: Mutex<VecDeque<NoiseData>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(VecDeque::new()),
        }
    }

    pub fn push(&self, data: NoiseData) -> Result<()> {
        self.data
            .lock()
            .map_err(|e| anyhow!(e.to_string()))?
            .push_back(data);
        Ok(())
    }
}
