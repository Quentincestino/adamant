# Adamant

Adamant is a x86 kernel that aims to be a microkernel, POSIX compliant and highly reliable.

## Dependencies

* [echfs](https://github.com/echfs/echfs) (check your package manager)
* [limine](https://github.com/limine-bootloader/limine)
* [Rust](https://rust-lang.org)

## About Rust

This kernel, like many others written in Rust, compiles with the Nightly version. I personally (Quentincestino) always use the latest nightly toolchain available, so I don't know about backward compatibility.

## Run

```shell=
# Gives execute rights to all scripts
chmod +x scripts/*.sh *.sh
# Build it and runs QEMU
./run_qemu.sh
# Build it and runs Bochs
./run_bochs.sh
```

## Build

If you somehow want to just build the kernel, do the following:
```shell=
# Basically calls cargo build
./scripts/build.sh
# Builds the bootable image with Limine
./scripts/build_limine.sh
```

You can find the bootable Limine image on the build directory.

## Licensing

I'm not a big fan of licenses since I'm a developer and not a lawyer. This project is licensed under the BSD 2-Clause License, wich seems permissive enough to my eyes. I basically just ask you to not be a jerk, and to at least pay respect to each other here.