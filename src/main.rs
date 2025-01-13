use std::env::consts::{ARCH, OS};
use std::time::{Duration, Instant};

#[cfg(windows)]
mod os {
    use super::*;
    use std::mem;
    use winapi::um::profileapi::QueryPerformanceFrequency;

    pub fn os() {
        let freq = unsafe {
            let mut freq_raw = mem::zeroed();
            QueryPerformanceFrequency(&mut freq_raw);
            *freq_raw.QuadPart() as u64
        };

        let resolution = Duration::from_nanos(1_000_000_000u64 / freq);

        println!(
            "OS: {:?} ({}/{}, QueryPerformanceFrequency = {})",
            resolution, OS, ARCH, freq
        );
    }
}

#[cfg(not(windows))]
mod os {
    use super::*;

    pub fn os() {
        println!("OS: not implemented ({}/{})", OS, ARCH);
    }
}

fn single_measure() -> Duration {
    let start = Instant::now();

    loop {
        let now = Instant::now();

        if now > start {
            return now - start;
        }
    }
}

fn measure() {
    const ITERATIONS: usize = 100;
    let mut min = Duration::MAX;

    for _ in 0..ITERATIONS {
        min = min.min(single_measure());
    }

    println!("Measured: {:?}", min);
}

fn main() {
    measure();
    os::os();
}
