A toy program for checking the resolution of Rust's `std::time::Instant`. Use only for entertainment.

```
git clone https://github.com/greeble-dev/instant_resolution.git
cd instant_resolution
cargo run --release
```
```
OS: windows/x86_64
Guessed: 100ns (QueryPerformanceFrequency = 10000000)
Measured: 100ns
```
```
OS: linux/x86_64
Guessed: 1ns (clock_getres(CLOCK_MONOTONIC) = 1)
Measured: 20ns
```
```
OS: macos/aarch64
Guessed: 42ns (clock_getres(CLOCK_UPTIME_RAW) = 42)
Measured: 41ns
```

The guessed time is from the OS API that we assume `Instant` is using. 

The measured time is from simply calling `Instant::now()` in a loop until it
increases, which might underestimate the resolution.
