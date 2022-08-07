# LPC550x

[![Documentation](https://docs.rs/lpc550x/badge.svg)](https://docs.rs/lpc550x)
[![Crates.io](https://img.shields.io/crates/v/lpc550x.svg)](https://crates.io/crates/lpc550x)

Register mappings for the NXP LPC550x/LPC55S0x family of Cortex-M33 microcontrollers generated with the `svd2rust` tool.

## User Manual

The complete user manual for this family of microcontrollers may be found at [UM11424][1] and is a useful supplement to this crate.

## Contribute

Install the `svd2rust` and `form` tools and run the `generate.sh` script to generate the crate from the SVD file. A line is also prepended to the `lib.rs` to prevent Clippy from checking the crate because the generated code does not lint well. Do not make manual edits to the `src` folder or the `build.rs` and `device.x` files which are all auto-generated.

[1]: https://www.nxp.com/docs/en/user-guide/UM11424.pdf
