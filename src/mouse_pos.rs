// Cargo.toml block inside comments (Rust 2024 / Script RFC)
---
[dependencies]
enigo = "0.3.0"
---

use enigo::{Enigo, Mouse, Settings};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[INFO] Tracking mouse position. Press Ctrl+C to stop.\n");

    let enigo = Enigo::new(&Settings::default())?;
    let mut last_pos = (-1, -1);

    loop {
        if let Ok((x, y)) = enigo.location() {
            if (x, y) != last_pos {
                print!("\rCurrent Position -> X: {:<5} Y: {:<5}", x, y);
                io::stdout().flush()?;
                last_pos = (x, y);
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}