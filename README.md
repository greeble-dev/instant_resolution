A toy program for checking the resolution of Rust's `std::time::Instant`. Use only for entertainment.

```
git clone https://github.com/greeble-dev/instant_resolution.git
cd instant_resolution
cargo run --release

Measured: 100ns
OS: 100ns (windows/x86_64, QueryPerformanceFrequency = 10000000)
```

The measured time is from simply calling `Instant::now()` in a loop until it
increases. The OS time is from taking a guess at the OS API that `Instant` might 
be using.
