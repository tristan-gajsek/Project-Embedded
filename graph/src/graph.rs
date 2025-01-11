use anyhow::Result;
use minifb::{Window, WindowOptions};
use plotters::{
    backend::BGRXPixel,
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea},
    style::RGBColor,
};
use std::{collections::VecDeque, sync::mpsc::Receiver};

use crate::{cli::Cli, data::NoiseData};

pub struct Graph {
    window: Window,
    buffer: Box<[u32]>,
    width: usize,
    height: usize,

    data: VecDeque<NoiseData>,
    receiver: Receiver<NoiseData>,
}

impl Graph {
    const BACKGROUND_COLOR: RGBColor = RGBColor(30, 30, 46);
    const PRIMARY_COLOR: RGBColor = RGBColor(205, 214, 244);
    const SECONDARY_COLOR: RGBColor = RGBColor(49, 50, 68);

    pub fn new(args: &Cli, receiver: Receiver<NoiseData>) -> Result<Self> {
        Ok(Self {
            window: Window::new(
                "Noise Data",
                args.width,
                args.height,
                WindowOptions::default(),
            )?,
            width: args.width,
            height: args.height,
            buffer: vec![0; args.width * args.height].into_boxed_slice(),
            data: VecDeque::new(),
            receiver,
        })
    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open()
    }

    pub fn update(&mut self) -> Result<()> {
        let mut count: usize = 0;
        self.receiver.try_iter().for_each(|data| {
            self.data.push_back(data);
            count += 1;
        });
        if count > 0 {
            self.update_buffer()?;
        }
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)?;
        Ok(())
    }

    fn update_buffer(&mut self) -> Result<()> {
        let bitmap = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
            bytemuck::cast_slice_mut(&mut self.buffer),
            (self.width as u32, self.height as u32),
        )?
        .into_drawing_area();
        bitmap.fill(&Self::BACKGROUND_COLOR)?;

        let mut chart = ChartBuilder::on(&bitmap)
            .x_label_area_size(100)
            .y_label_area_size(150)
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
        chart.draw_series(self.data.iter().map(|noise| noise.circle()))?;

        bitmap.present()?;
        Ok(())
    }
}
