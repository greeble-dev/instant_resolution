use std::env::consts::{ARCH, OS};
use std::time::{Duration, Instant};

#[cfg(target_family = "windows")]
mod os {
    use super::*;
    use std::mem;
    use windows_sys::Win32::System::Performance::QueryPerformanceFrequency;

    pub fn os() {
        let freq = unsafe {
            let mut freq_raw = mem::zeroed();
            QueryPerformanceFrequency(&mut freq_raw);
            freq_raw as u64
        };

        let resolution = Duration::from_nanos(1_000_000_000u64 / freq);

        println!(
            "OS: {:?} ({}/{}, QueryPerformanceFrequency = {})",
            resolution, OS, ARCH, freq
        );
    }
}

#[cfg(target_family = "unix")]
mod os {
    use super::*;
    use libc::{clock_getres, CLOCK_MONOTONIC};
    use std::mem;

    pub fn os() {
        let resolution_ns = unsafe {
            let mut tp = mem::zeroed();
            clock_getres(CLOCK_MONOTONIC, &mut tp);
            tp.tv_nsec as u64
        };

        let resolution = Duration::from_nanos(resolution_ns);

        println!(
            "OS: {:?} ({}/{}, clock_getres(CLOCK_MONOTONIC) = {})",
            resolution, OS, ARCH, resolution_ns
        );
    }
}

#[cfg(not(any(target_family = "windows", target_family = "unix")))]
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
