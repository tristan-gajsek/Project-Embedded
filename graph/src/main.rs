use std::{sync::Arc, thread};

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Source};
use colored::Colorize;
use graph::Graph;
use minifb::{Window, WindowOptions};

mod cli;
mod data;
mod graph;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {e}", "error:".red());
    }
}

fn run() -> Result<()> {
    let args = Cli::parse();
    let args2 = args.clone();
    let graph = Arc::new(Graph::new());
    let graph2 = Arc::clone(&graph);

    let mut window = Window::new("", args.width, args.height, WindowOptions::default())?;
    let mut buffer = [0u32, (args.width * args.height) as u32];
    let data_thread = thread::spawn(move || match args2.source {
        Source::SerialPort => data::read_serial_port(&args2, graph2),
        Source::Input => data::read_input(graph2),
        Source::Random => data::generate_random(graph2),
    });

    while window.is_open() {
        graph.draw(&args, bytemuck::cast_slice_mut(&mut buffer))?;
        window.update_with_buffer(&buffer, args.width, args.height)?;
    }
    data_thread.join().expect("Error when joining thread")?;

    Ok(())
}
