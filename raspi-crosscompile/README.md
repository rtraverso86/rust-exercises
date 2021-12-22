# RUST Binary Program with Raspberry Pi 4 Cross-Compilation

The following example project follows the [steps here](https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050).

## Ubuntu Dependencies

 * gcc-arm-linux-gnueabihf (ARMv7 32bit)

## Raspi 4 and Target Support

The [rustc book](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
contains a list of supported targets.

The Raspberry 4 uses an [ARM Cortex-A72](https://en.wikipedia.org/wiki/ARM_Cortex-A72),
which has a 64-bit ARMv8-A architecture.

Now, this sample uses a 32bit target architecture with arm7 but, once I get hold of
an actual unicor- I mean, raspi4, I will finally be able to test whether there's a
more appropriate target for a 64 bits OS.
