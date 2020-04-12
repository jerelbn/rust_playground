// This is a simple numerical integration comparison just to get familiar with Rust.
mod utils;
use utils::utils::round2dec;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Time parameters
    let dt = 0.1;
    let t0 = 0.0;
    let tf = 1.0;
    let mut ts: Vec<f64> = Vec::with_capacity(((tf / dt) as usize) + 1);
    ts.push(t0);
    for i in 1..ts.capacity() {
        ts.push(round2dec(ts[i - 1] + dt, 6));
    }

    // Collect true function and true derivative
    let mut true_fs: Vec<f64> = Vec::new();
    let mut true_dfs: Vec<f64> = Vec::new();
    for t in ts.iter() {
        true_fs.push(f(t));
        true_dfs.push(df(t));
    }

    // Log truth to file
    let mut file = File::create("/tmp/true_fs.log").expect("Unable to open /tmp/true_fs.log");
    for f in true_fs.iter() {
        file.write(unsafe { std::slice::from_raw_parts((f as *const f64) as *const u8, std::mem::size_of::<f64>()) })
            .expect("Unable to write to /tmp/true_fs.log");
    }
    Ok(())
}

fn f(t: &f64) -> f64 {
    t.sin() * t.cos()
}

fn df(t: &f64) -> f64 {
    -t.cos() * t.sin()
}