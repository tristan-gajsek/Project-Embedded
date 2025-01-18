use std::{io::Cursor, sync::mpsc::Sender, time::Duration};

use crate::cli::Cli;
use anyhow::{bail, Result};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::ops::Range;

#[derive(Clone, Copy)]
pub enum Data {
    NoiseData(NoiseData),
    MagnetometerData(MagnetometerData),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NoiseData {
    pub latitude: f64,
    pub longitude: f64,
    pub decibels: f64,
}

impl NoiseData {
    pub const LATITUDE_RANGE: Range<f64> = -90.0..90.0;
    pub const LONGITUDE_RANGE: Range<f64> = -180.0..180.0;
    pub const DECIBEL_RANGE: Range<f64> = 50.0..150.0;

    fn parse(data: &[u8]) -> Result<Self> {
        let mut data = Cursor::new(data);
        Ok(NoiseData {
            latitude: data.read_f64::<LittleEndian>()?,
            longitude: data.read_f64::<LittleEndian>()?,
            decibels: data.read_f64::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MagnetometerData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl MagnetometerData {
    pub const RANGE: Range<f64> = -1.3..1.3;

    fn parse(data: &[u8]) -> Result<Self> {
        let mut data = Cursor::new(data);
        Ok(MagnetometerData {
            x: data.read_i16::<BigEndian>()? as f64 / 1100.0,
            y: data.read_i16::<BigEndian>()? as f64 / 1100.0,
            z: data.read_i16::<BigEndian>()? as f64 / 980.0,
        })
    }
}

pub fn read_serial_port(args: &Cli, sender: Sender<Data>) -> Result<()> {
    let timeout = args.timeout.map_or(Duration::MAX, Duration::from_secs);
    let mut port = serialport::new(args.path.as_ref(), args.baud_rate)
        .timeout(timeout)
        .open()?;

    loop {
        let mut header = [0; 2];
        port.read_exact(&mut header)?;
        sender.send(match u16::from_le_bytes(header) {
            0xABCD => {
                let mut buffer = [0; 3 * size_of::<f64>()];
                port.read_exact(&mut buffer)?;
                Data::NoiseData(NoiseData::parse(&buffer)?)
            }
            0xBBCD => {
                let mut buffer = [0; 3 * size_of::<u16>()];
                port.read_exact(&mut buffer)?;
                Data::MagnetometerData(MagnetometerData::parse(&buffer)?)
            }
            _ => bail!("Invalid header"),
        })?;
    }
}
