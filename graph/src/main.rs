use std::{process, sync::mpsc, thread};

use anyhow::{Error, Result};
use clap::Parser;
use cli::{Cli, Source};
use colored::Colorize;
use graph::Graph;

mod cli;
mod data;
mod graph;

fn main() {
    if let Err(e) = run() {
        abort(e);
    }
}

fn abort(e: Error) {
    eprintln!("{} {e}", "error:".red());
    process::exit(1);
}

fn run() -> Result<()> {
    let args = Cli::parse();
    let (sender, receiver) = mpsc::channel();
    let mut graph = Graph::new(&args, receiver)?;

    let data_thread = thread::spawn(move || {
        if let Err(e) = match args.source {
            Source::SerialPort => data::read_serial_port(&args, sender),
            Source::Input => data::read_input(sender),
            Source::Random => data::generate_random(&args, sender),
        } {
            abort(e);
        }
    });

    graph.update_buffer()?;
    while !graph.should_close() {
        graph.update()?;
    }
    data_thread.join().expect("Error when joining thread");
    Ok(())
}
