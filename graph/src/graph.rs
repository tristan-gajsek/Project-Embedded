use anyhow::Result;
use minifb::{Window, WindowOptions};
use plotters::{
    backend::BGRXPixel,
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, IntoDrawingArea},
    series::LineSeries,
    style::{self, Color, RGBAColor, RGBColor},
};
use std::{collections::VecDeque, sync::mpsc::Receiver};

use crate::{
    cli::Cli,
    data::{Data, MagnetometerData, NoiseData},
};

enum DataType {
    Noise,
    Magnetometer,
}

pub struct Graph {
    window: Window,
    buffer: Box<[u32]>,
    noise_data: Vec<NoiseData>,
    magnetometer_data: VecDeque<MagnetometerData>,
    next_draw: Option<DataType>,
    receiver: Receiver<Data>,
}

impl Graph {
    const BACKGROUND_COLOR: RGBColor = RGBColor(30, 30, 46);
    const PRIMARY_COLOR: RGBColor = RGBColor(205, 214, 244);
    const SECONDARY_COLOR: RGBColor = RGBColor(49, 50, 68);

    pub fn new(args: &Cli, receiver: Receiver<Data>) -> Result<Self> {
        Ok(Self {
            window: Window::new(
                "Noise Data",
                args.width,
                args.height,
                WindowOptions::default(),
            )?,
            buffer: vec![0; args.width * args.height].into_boxed_slice(),
            noise_data: vec![],
            magnetometer_data: VecDeque::new(),
            next_draw: None,
            receiver,
        })
    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open()
    }

    pub fn update(&mut self) -> Result<()> {
        let (width, height) = self.window.get_size();

        self.receiver.try_iter().for_each(|data| match data {
            Data::NoiseData(data) => {
                self.noise_data.push(data);
                self.next_draw = Some(DataType::Noise);
            }
            Data::MagnetometerData(data) => {
                self.noise_data.clear();
                self.magnetometer_data.push_back(data);
                if self.magnetometer_data.len() > 100 {
                    self.magnetometer_data.pop_front();
                }
                self.next_draw = Some(DataType::Magnetometer);
            }
        });

        self.draw()?;
        self.window
            .update_with_buffer(&self.buffer, width, height)?;
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        match self.next_draw {
            Some(DataType::Noise) => self.draw_noise_graph()?,
            Some(DataType::Magnetometer) => self.draw_magnetometer_graph()?,
            None => (),
        }
        self.next_draw = None;
        Ok(())
    }

    pub fn draw_noise_graph(&mut self) -> Result<()> {
        let (width, height) = self.window.get_size();
        let bitmap = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
            bytemuck::cast_slice_mut(&mut self.buffer),
            (width as u32, height as u32),
        )?
        .into_drawing_area();
        bitmap.fill(&Self::BACKGROUND_COLOR)?;

        let mut chart = ChartBuilder::on(&bitmap)
            .x_label_area_size(128)
            .y_label_area_size(128)
            .margin(10)
            .build_cartesian_2d(NoiseData::LATITUDE_RANGE, NoiseData::LONGITUDE_RANGE)?;
        chart
            .configure_mesh()
            .axis_style(Self::PRIMARY_COLOR)
            .bold_line_style(Self::PRIMARY_COLOR)
            .light_line_style(Self::SECONDARY_COLOR)
            .label_style(("sans-serif", 32, &Self::PRIMARY_COLOR))
            .x_labels(10)
            .y_labels(10)
            .x_desc("Latitude")
            .y_desc("Longitude")
            .draw()?;

        chart.draw_series(self.noise_data.iter().map(|data| {
            let (min, max) = (NoiseData::DECIBEL_RANGE.start, NoiseData::DECIBEL_RANGE.end);
            let r = (data.decibels.clamp(min, max) - min) / (max - min);
            let g = 1.0 - r;
            let style = RGBAColor(
                (r * u8::MAX as f64) as u8,
                (g * u8::MAX as f64) as u8,
                0,
                0.5,
            )
            .filled();
            Circle::new((data.latitude, data.longitude), data.decibels / 5.0, style)
        }))?;

        bitmap.present()?;
        Ok(())
    }

    pub fn draw_magnetometer_graph(&mut self) -> Result<()> {
        let (width, height) = self.window.get_size();
        let bitmap = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
            bytemuck::cast_slice_mut(&mut self.buffer),
            (width as u32, height as u32),
        )?
        .into_drawing_area();
        bitmap.fill(&Self::BACKGROUND_COLOR)?;

        let mut chart = ChartBuilder::on(&bitmap)
            .x_label_area_size(128)
            .y_label_area_size(128)
            .margin(10)
            .build_cartesian_2d(
                0.0..self.magnetometer_data.len() as f64,
                MagnetometerData::RANGE,
            )?;
        chart
            .configure_mesh()
            .axis_style(Self::PRIMARY_COLOR)
            .bold_line_style(Self::PRIMARY_COLOR)
            .light_line_style(Self::SECONDARY_COLOR)
            .label_style(("sans-serif", 32, &Self::PRIMARY_COLOR))
            .x_labels(10)
            .y_labels(10)
            .x_desc("Latitude")
            .y_desc("Longitude")
            .draw()?;

        chart.draw_series(LineSeries::new(
            self.magnetometer_data
                .iter()
                .enumerate()
                .map(|(i, data)| (i as f64, data.x)),
            style::RED.stroke_width(2),
        ))?;
        chart.draw_series(LineSeries::new(
            self.magnetometer_data
                .iter()
                .enumerate()
                .map(|(i, data)| (i as f64, data.y)),
            style::GREEN.stroke_width(2),
        ))?;
        chart.draw_series(LineSeries::new(
            self.magnetometer_data
                .iter()
                .enumerate()
                .map(|(i, data)| (i as f64, data.z)),
            style::BLUE.stroke_width(2),
        ))?;

        bitmap.present()?;
        Ok(())
    }
}
