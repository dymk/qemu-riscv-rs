# Embassy QEMU-RISCV HAL
An Embassy HAL for the ODP custom [RISC-V EC machine in QEMU](https://github.com/kurtjd/qemu/blob/odp/hw/riscv/ec.c).

Run native unit tests from the repository root:

```sh
cargo test -p embassy-qemu-riscv --lib --no-default-features
```

The I2C target tests exercise blocking and async receive boundaries, including
a final RX byte pending at STOP, and normal read completion.