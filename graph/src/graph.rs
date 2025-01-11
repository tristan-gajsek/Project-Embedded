use anyhow::{anyhow, Result};
use std::{collections::VecDeque, sync::Mutex};

use crate::{cli::Cli, data::NoiseData};

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

    pub fn draw(&self, args: &Cli, buffer: &mut [u8]) -> Result<()> {
        Ok(())
    }
}
