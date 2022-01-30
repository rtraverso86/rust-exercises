# RUST Binary Program with Home Assistant on Raspberry Pi 4 Cross-Compilation

## Raspi 4 and Target Support

The [rustc book](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
contains a list of supported targets.

The Raspberry 4 uses an [ARM Cortex-A72](https://en.wikipedia.org/wiki/ARM_Cortex-A72),
which has a 64-bit ARMv8-A architecture.

## Home Assistant Addons

Home Assistant (HA) addons run on an Alpine Linux docker containers, where native apps should
be linked against MUSL instead of GLIBC.

The following is the output of `uname -a` on such a container:

```
Linux local-hello-world 5.10.63-v8 #1 SMP PREEMPT Mon Jan 17 17:02:49 UTC 2022 aarch64 Linux
```

## Cross-Compilation for HA Addons

Make sure to have Docker installed. Then, install `cross`:

```bash
cargo install cross
```

You may now compile the target like this:

```bash
cross build --release --target=aarch64-unknown-linux-musl
```

## Notes

I still could not get the plain old `cargo build --target=aarch64-unknown-linux-musl` working.
