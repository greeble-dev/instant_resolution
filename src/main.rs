use std::env::consts::{ARCH, OS};
use std::time::{Duration, Instant};

#[cfg(target_family = "windows")]
mod os {
    use super::*;
    use windows_sys::Win32::System::Performance::QueryPerformanceFrequency;

    pub fn os() {
        let freq_ns = unsafe {
            let mut freq_raw = std::mem::zeroed();
            QueryPerformanceFrequency(&mut freq_raw);
            freq_raw as u64
        };

        let resolution = Duration::from_nanos(1_000_000_000u64 / freq_ns);

        println!("OS: {resolution:?} ({OS}/{ARCH}, QueryPerformanceFrequency = {freq_ns})");
    }
}

#[cfg(target_family = "unix")]
mod os {
    use super::*;

    pub fn os() {
        // See `std/sys/pal/unix/time.rs`, `Instant::now`.
        #[cfg(target_vendor = "apple")]
        let (clock_id, clock_name) = (libc::CLOCK_UPTIME_RAW, "CLOCK_UPTIME_RAW");
        #[cfg(not(target_vendor = "apple"))]
        let (clock_id, clock_name) = (libc::CLOCK_MONOTONIC, "CLOCK_MONOTONIC");

        let resolution_ns = unsafe {
            let mut tp = std::mem::zeroed();
            libc::clock_getres(clock_id, &mut tp);
            tp.tv_nsec as u64
        };

        let resolution = Duration::from_nanos(resolution_ns);

        println!("OS: {resolution:?} ({OS}/{ARCH}, clock_getres({clock_name}) = {resolution_ns})");
    }
}

#[cfg(not(any(target_family = "windows", target_family = "unix")))]
mod os {
    use super::*;

    pub fn os() {
        println!("OS: not implemented ({OS}/{ARCH})");
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
