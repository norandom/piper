use std::{
    fs::{self, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use anyhow::{Context, Result};
use clap::Parser;

const MB: usize = 1024 * 1024;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Size of the ring buffer in MB
    #[arg(short = 's', long, default_value = "5")]
    size: usize,
}

struct Piper {
    buffer: Vec<String>,
    max_size: usize,
    current_size: usize,
    backup_path: PathBuf,
}

impl Piper {
    fn new(size_mb: usize) -> Result<Self> {
        let piper_dir = PathBuf::from(".piper");
        fs::create_dir_all(&piper_dir)?;

        Ok(Self {
            buffer: Vec::new(),
            max_size: size_mb * MB,
            current_size: 0,
            backup_path: piper_dir.join("buffer_backup"),
        })
    }

    fn add_line(&mut self, line: String) {
        let line_size = line.len();

        while self.current_size + line_size > self.max_size && !self.buffer.is_empty() {
            let removed = self.buffer.remove(0);
            self.current_size -= removed.len();
        }

        self.current_size += line_size;
        self.buffer.push(line);
    }

    fn write_backup(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.backup_path)
            .context("Failed to create backup file")?;

        for line in &self.buffer {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let piper = Arc::new(Mutex::new(Piper::new(args.size)?));

    // Set up Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let p = piper.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        if let Ok(piper) = p.lock() {
            let _ = piper.write_backup();
        }
    })?;

    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    for line in reader.lines() {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        let line = line?;
        {
            let mut piper = piper.lock().unwrap();
            piper.add_line(line.clone());
        }
        println!("{}", line);
    }

    // Write backup on normal exit too
    if let Ok(piper) = piper.lock() {
        piper.write_backup()?;
    }

    Ok(())
}
