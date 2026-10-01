# Embassy QEMU-RISCV HAL
An Embassy HAL for the ODP custom [RISC-V EC machine in QEMU](https://github.com/kurtjd/qemu/blob/odp/hw/riscv/ec.c).

Run native unit tests from the repository root:

```sh
cargo test -p embassy-qemu-riscv --lib --no-default-features
```

The I2C target tests exercise blocking and async receive boundaries, including
a final RX byte pending at STOP, and normal read completion.

`gpio::Input::is_initialized()` reads QEMU's per-pin validity mask at GPIO
offset `0x18`. A low input level alone does not establish that an external
source has been configured. Use `wait_for_high`/`wait_for_low` to follow
persistent levels; those waits do not count transient edges between polls.