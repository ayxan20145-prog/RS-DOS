# RS-DOS

![example](example.png)

a simple OS written in rust

## Build

Requirements:
- `nightly rust`
- `cargo-make`
- `grub`
- `xorriso`
- `qemu (optional for testing)`

Commands:
- `cargo make iso` builds `RS-DOS.iso`
- `cargo make run` runs `RS-DOS.iso` in qemu
- `cargo make clean` removes build files
