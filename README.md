# Carbon Monoxide Sensor

A program for the [STM32F3DISCOVERY](https://www.st.com/en/evaluation-tools/stm32f3discovery.html) board to display (SSD1306) the amount of Carbon Monoxide (CO)
detected by a sensor (MQ-7 + ADS1115).

## Development

Requirements include [Embedded Rust](https://rust-embedded.github.io/book/), OpenOCD, and GDB.

In one terminal run `openocd` from this directory. In a second terminal run `cargo run`. Once in the debugger
`continue` and then `next` until the program is in its main loop.
