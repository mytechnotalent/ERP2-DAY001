<img src="https://github.com/mytechnotalent/DAY001/blob/main/DAY001.png?raw=true">

## FREE Reverse Engineering Self-Study Course [HERE](https://github.com/mytechnotalent/Reverse-Engineering-Tutorial)
VIDEO PROMO [HERE](https://www.youtube.com/watch?v=aD7X9sXirF8)

<br>

# DAY001: Blink a Single LED ⭐

**Difficulty**: Beginner  
**Date**: Day 1 of 365  
**Components**: LED, Resistor  
**Concepts**: GPIO, Digital Output, Async Programming

---

## 📋 Project Overview

This is the first project in the **365 Pico2 RP2350 Project Ideas** series. We're implementing the classic "Hello World" of embedded systems: blinking an LED. This simple project introduces you to Embassy Rust on the Raspberry Pi Pico 2 and establishes the foundation for all future projects.

### What You'll Learn

- Setting up an Embassy Rust project for RP2350
- Configuring GPIO pins as digital outputs
- Using Embassy's async/await for timing
- Understanding the embedded development workflow
- Using defmt for efficient embedded logging
- Working with probe-rs for flashing and debugging

---

## 🔌 Hardware Requirements

### Components Needed

| Quantity | Component                    | Notes                                 |
| -------- | ---------------------------- | ------------------------------------- |
| 1        | Raspberry Pi Pico 2 (RP2350) |                                       |
| 1        | LED (any color)              | Red, Yellow, Green, or White from kit |
| 1        | 100Ω Resistor                | Current-limiting resistor             |
| 1        | Breadboard                   | For prototyping                       |
| 2        | Jumper Wires                 | Male-to-Male                          |

### Wiring Diagram

```
┌─────────────────────────┐
│  Raspberry Pi Pico 2    │
│                         │
│  ┌────────┐             │
│  │  GP25  ├─────┐       │  (or use GP15 for external LED)
│  └────────┘     │       │
│                 │       │
│  ┌────────┐     │       │
│  │  GND   ├──┐  │       │
│  └────────┘  │  │       │
└──────────────┼──┼───────┘
               │  │
               │  └──[100Ω]──┬──[LED]──┐
               │             │    +    │
               └─────────────┴─────────┘
                             │    -    │
                             └─────────┘

Connection Steps:
1. GP25 (Pin 25) → 100Ω Resistor → LED Anode (longer leg, +)
2. LED Cathode (shorter leg, -) → GND

Note: The Pico 2 has a built-in LED on GP25, so you can skip
      the external circuit initially and just run the code!
```

### Pin Information

- **GP25**: Built-in LED on Pico 2 (perfect for testing)
- **GP15**: Alternative GPIO pin for external LED
- **GND**: Ground connection (any GND pin works)

---

## 🛠️ Software Requirements

### Prerequisites

1. **Rust Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **ARM Cortex-M Target**
   ```bash
   rustup target add thumbv8m.main-none-eabihf
   ```

3. **probe-rs** (for flashing and debugging)
   ```bash
   cargo install probe-rs-tools --locked
   ```

4. **flip-link** (stack overflow protection)
   ```bash
   cargo install flip-link
   ```

### Dependencies

All dependencies are specified in `Cargo.toml`:

- **embassy-executor**: Async task executor for embedded systems (git version)
- **embassy-time**: Time and timer abstractions (git version)
- **embassy-rp**: Hardware Abstraction Layer (HAL) for RP2350 with `rp235xa` chip feature (git version for full RP2350 support)
- **cortex-m**: Low-level Cortex-M utilities
- **defmt**: Efficient logging framework for embedded systems (version 1.0)
- **defmt-rtt**: RTT transport for defmt logging (version 1.0)
- **panic-probe**: Panic handler for debugging

> **Important Note**: We're using git versions of the Embassy framework because the crates.io releases don't yet have full RP2350 support. The RP2350 uses ARMv8-M architecture with different MPU registers than earlier chips. We specifically enable the `rp235xa` feature for Pico 2 (RP2350-A revision) and `critical-section-impl` for proper interrupt handling.

---

## 📝 Code Structure

### Project Files

```
DAY001/
├── Cargo.toml           # Project dependencies and configuration
├── build.rs             # Build script for linker configuration
├── memory.x             # Memory layout for RP2350
├── .cargo/
│   └── config.toml      # Target and runner configuration
├── src/
│   └── main.rs          # Main application code
└── README.md            # This file
```

### Key Code Sections

#### 1. Main Function

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut led = Output::new(peripherals.PIN_25, Level::Low);
    
    loop {
        led.set_high();
        Timer::after_millis(1000).await;
        led.set_low();
        Timer::after_millis(1000).await;
    }
}
```

**Explanation**:
- `embassy_rp::init()`: Initializes the RP2350 hardware
- `Output::new()`: Configures GP25 as an output, starting LOW (off)
- `led.set_high()`: Sets pin to 3.3V (LED on)
- `led.set_low()`: Sets pin to 0V (LED off)
- `Timer::after_millis()`: Async delay that doesn't block

#### 2. Memory Configuration (`memory.x`)

Defines the RP2350's memory regions:
- **BOOT2** (256 bytes): Second-stage bootloader
- **FLASH** (4MB): Program storage
- **RAM** (520KB): Runtime memory

#### 3. Build Configuration (`.cargo/config.toml`)

- Target: `thumbv8m.main-none-eabihf` (ARMv8-M for RP2350)
- Runner: `probe-rs run --chip RP2350`
- Linker: `flip-link` for stack protection

---

## 🚀 Building and Flashing

### Step 1: Connect the Pico 2

1. Hold the **BOOTSEL** button on the Pico 2
2. Connect the Pico to your computer via USB
3. Release the BOOTSEL button
4. The Pico appears as a USB drive (RPI-RP2)

### Step 2: Build the Project

```bash
cd DAY001
cargo build --release
```

This compiles the code for the RP2350 target.

### Step 3: Flash and Run

```bash
cargo run --release
```

This will:
1. Compile the code
2. Flash it to the Pico 2 using probe-rs
3. Start RTT logging to show debug output

### Expected Output

You should see in your terminal:

```
INFO  DAY001: Blink LED - Starting!
INFO  Embassy Rust on Raspberry Pi Pico 2 (RP2350)
INFO  LED initialized on GPIO 25
INFO  Starting blink loop with 1 second interval...
INFO  LED ON (blink #0)
INFO  LED OFF
INFO  LED ON (blink #1)
INFO  LED OFF
...
```

And the LED will blink: ON for 1 second, OFF for 1 second, repeating forever.

---

## 🔧 Troubleshooting

### Issue: `probe-rs` not found

**Solution**: Install probe-rs tools
```bash
cargo install probe-rs-tools --locked
```

### Issue: Can't find device

**Solution**: 
1. Ensure BOOTSEL was pressed during connection
2. Try a different USB cable (some are power-only)
3. Check USB permissions on Linux:
   ```bash
   sudo usermod -a -G plugdev $USER
   ```

### Issue: Build errors about missing target

**Solution**: Add the ARM target
```bash
rustup target add thumbv8m.main-none-eabihf
```

### Issue: LED doesn't blink

**Solutions**:
1. Check the wiring (resistor and polarity)
2. Verify you're using the correct GPIO pin
3. Try the built-in LED first (GP25, no wiring needed)
4. Check if the LED is functional (test with a battery)

### Issue: Linker errors

**Solution**: Install flip-link
```bash
cargo install flip-link
```

---

## 📚 Understanding the Code

### What is Embassy?

Embassy is a modern async framework for embedded Rust. It provides:
- **Async/await**: Write concurrent code that looks sequential
- **Efficient timers**: Hardware-accelerated timing
- **Type safety**: Rust's compile-time guarantees for embedded systems
- **Low power**: Efficient task scheduling and sleep modes

### Why Async?

Traditional embedded code uses blocking delays:
```rust
delay_ms(1000);  // CPU does nothing for 1 second
```

Embassy's async approach:
```rust
Timer::after_millis(1000).await;  // CPU can do other tasks
```

This allows multiple tasks to run concurrently on a single core!

### GPIO Basics

- **Digital Output**: Pin is either HIGH (3.3V) or LOW (0V)
- **Current Limit**: RP2350 pins source ~12mA max
- **Resistor**: Limits LED current to safe levels
  - LED forward voltage: ~2V
  - Desired current: ~10mA
  - Resistor: (3.3V - 2V) / 0.01A = 130Ω (100Ω works fine)

### defmt Logging

`defmt` is a highly efficient logging framework:
- Zero-cost abstractions
- Formatting happens on the host computer, not the microcontroller
- Minimal flash and RAM usage
- Perfect for embedded systems

---

## 🎯 Experiments and Modifications

### 1. Change Blink Speed

Modify the delay values in `main.rs`:

```rust
// Fast blink (0.2 seconds)
Timer::after_millis(200).await;

// Slow blink (3 seconds)
Timer::after_millis(3000).await;
```

### 2. Different Blink Pattern

Create an SOS pattern:

```rust
// S: three short blinks
for _ in 0..3 {
    led.set_high();
    Timer::after_millis(200).await;
    led.set_low();
    Timer::after_millis(200).await;
}

// O: three long blinks
for _ in 0..3 {
    led.set_high();
    Timer::after_millis(600).await;
    led.set_low();
    Timer::after_millis(200).await;
}

// S: three short blinks
for _ in 0..3 {
    led.set_high();
    Timer::after_millis(200).await;
    led.set_low();
    Timer::after_millis(200).await;
}

Timer::after_millis(2000).await;  // Pause between SOS
```

### 3. Use External LED

Change the pin in `main.rs`:

```rust
let mut led = Output::new(peripherals.PIN_15, Level::Low);
```

Wire an external LED to GP15.

### 4. Adjust Logging Level

In `.cargo/config.toml`, change:

```toml
DEFMT_LOG = "debug"  # More verbose
# or
DEFMT_LOG = "warn"   # Less verbose
```

---

## 🧪 Challenges

1. **Heartbeat Pattern**: Make the LED pulse like a heartbeat
2. **Multiple Speeds**: Use a pattern that cycles through different blink speeds
3. **Binary Counter**: Use 4 LEDs to count in binary (preview of Day 7)
4. **Morse Code**: Transmit your name in Morse code (preview of Day 10)

---

## 📖 Next Steps

Once you've mastered blinking a single LED, you're ready for:

- **DAY002**: Blink multiple LEDs in sequence
- **DAY003**: Traffic light simulation
- **DAY004**: LED brightness control with PWM

---

## 🔗 Resources

### Official Documentation

- [Embassy Book](https://embassy.dev/book/)
- [embassy-rp Documentation](https://docs.embassy.dev/embassy-rp/)
- [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
- [Pico 2 Documentation](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html)

### Rust Embedded

- [Embedded Rust Book](https://rust-embedded.github.io/book/)
- [probe-rs Documentation](https://probe.rs/)
- [defmt Book](https://defmt.ferrous-systems.com/)

### Community

- [Embassy GitHub](https://github.com/embassy-rs/embassy)
- [Rust Embedded Matrix Chat](https://matrix.to/#/#rust-embedded:matrix.org)

---

## 📄 License

MIT License - Copyright (c) 2025

---

## ✅ Completion Checklist

- [ ] Hardware assembled correctly
- [ ] Rust toolchain installed
- [ ] probe-rs and flip-link installed
- [ ] Project builds without errors
- [ ] LED blinks at 1Hz (1 second on, 1 second off)
- [ ] RTT logging displays in terminal
- [ ] Experimented with different blink speeds
- [ ] Understood async/await in Embassy
- [ ] Ready to move to DAY002

---

**Congratulations!** 🎉 You've completed your first Embassy Rust project on the Pico 2. This simple blink is the foundation for all 365 projects ahead. The skills you learned today—GPIO control, async programming, and the build/flash workflow—will be used throughout this journey.

*"A journey of 365 projects begins with a single blink."* 🦀

---

**Project Status**: ⭐ Beginner | **Estimated Time**: 30-60 minutes | **Success Rate**: 99%
