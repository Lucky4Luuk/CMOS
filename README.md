# CMOS

![crates.io](https://img.shields.io/crates/v/cmos-rtc.svg)
![docs.rs](https://docs.rs/cmos-rtc/badge.svg)

"CMOS" is a tiny bit of very low power static memory that lives on the same chip as the Real-Time Clock (RTC)

## Usage
```rust
use cmos_rtc::{ReadRTC, Time};

let mut cmos = ReadRTC::new(0x00, 0x00);
let time: Time = cmos.read();
```
