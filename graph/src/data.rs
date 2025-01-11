use std::{io, sync::Arc, thread, time::Duration};

use crate::{cli::Cli, graph::Graph};
use anyhow::{bail, Result};
use colored::Colorize;
use rand::{random, thread_rng};

#[derive(Debug, PartialEq)]
pub struct NoiseData {
    pub latitude: f64,
    pub longitude: f64,
    pub decibels: f64,
}

impl NoiseData {
    pub fn new(latitude: f64, longitude: f64, decibels: f64) -> Self {
        Self {
            latitude,
            longitude,
            decibels,
        }
    }

    fn parse(input: &str) -> Result<Self> {
        let nums = input
            .split_whitespace()
            .map(|s| s.parse().map_err(anyhow::Error::from))
            .collect::<Result<Vec<_>>>()?;
        if nums.len() != 3 {
            bail!("Couldn't parse noise data");
        }
        Ok(NoiseData {
            latitude: nums[0],
            longitude: nums[1],
            decibels: nums[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::data::NoiseData;

    #[test]
    fn data_parsing() {
        assert_eq!(
            NoiseData::new(1.0, 2.0, 3.0),
            NoiseData::parse("1 2 3").expect("Couldn't parse data"),
        );
        matches!(NoiseData::parse(" 1 2 3 "), Err(_));
    }
}

pub fn read_serial_port(args: Cli, graph: Arc<Graph>) -> Result<()> {
    let port = serialport::new(args.path.as_ref(), args.baud_rate)
        .timeout(
            args.timeout
                .map_or(Duration::MAX, |s| Duration::from_secs(s)),
        )
        .open()?;
    loop {
        unimplemented!()
    }
}

pub fn read_input(graph: Arc<Graph>) -> Result<()> {
    loop {
        let mut input = "".to_string();
        io::stdin().read_line(&mut input)?;
        match NoiseData::parse(input.trim()) {
            Ok(data) => graph.push(data)?,
            Err(e) => eprintln!("{} {e}", "error".red()),
        }
    }
}

pub fn generate_random(graph: Arc<Graph>) -> Result<()> {
    loop {
        thread::sleep(Duration::from_secs(1));
        graph.push(NoiseData::new(random(), random(), random()))?;
    }
}
