use std::{io, sync::Arc, time::Duration};

use crate::{cli::Cli, graph::Graph};
use anyhow::{Ok, Result};
use colored::Colorize;
use nom::{
    character::complete::multispace0, combinator::map_res, number::complete::double,
    sequence::tuple, IResult,
};

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

    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(
            tuple((
                multispace0,
                double,
                multispace0,
                double,
                multispace0,
                double,
                multispace0,
            )),
            |(_, lat, _, long, _, dec, _)| Ok(Self::new(lat, long, dec)),
        )(input)
    }
}

pub fn read_serial_port(args: &Cli, graph: Arc<Graph>) -> Result<()> {
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

pub fn simulate(graph: Arc<Graph>) -> Result<()> {
    unimplemented!();
    Ok(())
}

pub fn read_input(graph: Arc<Graph>) -> Result<()> {
    loop {
        let mut input = "".to_string();
        io::stdin().read_line(&mut input)?;
        match NoiseData::parse(&input) {
            IResult::Ok((_, data)) => graph.push(data)?,
            IResult::Err(e) => eprintln!("{} {e}", "error:".red()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::NoiseData;

    #[test]
    fn data_parsing() {
        assert_eq!(
            NoiseData::new(1.0, 2.0, 3.0),
            NoiseData::parse(" 1 2 3 ").expect("Couldn't parse data").1,
        );
        NoiseData::parse("a b c").expect_err("Successfully parsed invalid data");
    }
}
