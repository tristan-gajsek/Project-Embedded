use anyhow::Result;
use minifb::{Window, WindowOptions};
use plotters::{
    backend::BGRXPixel,
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, IntoDrawingArea},
    style::{self, Color},
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
    pub fn new(args: &Cli, receiver: Receiver<NoiseData>) -> Result<Self> {
        Ok(Self {
            window: Window::new("Graph", args.width, args.height, WindowOptions::default())?,
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
        Ok(())
    }
}
