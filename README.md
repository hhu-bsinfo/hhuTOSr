# Isolation und Schutz in Betriebssystemen

Der Kurs "Isolation und Schutz in Betriebssystemen" startet mit [Aufgabe 8](https://github.com/hhu-bsinfo/hhuTOSr/tree/aufgabe-8). Falls Sie am Kurs "Betriebssystementwicklung" teilgenommen haben, können Sie ihr bestehendes System weiter entwickeln. Ansonsten findet sich im Branch [vorgabe-8](https://github.com/hhu-bsinfo/hhuTOSr/tree/vorgabe-8) ein fertiges Basis-System, dass als Grundlage für den Kurs verwendet werden kann.

# hhuTOS

hhuTOS = hhu Teaching Operating System.

This file describes all commands needed for building, running, and debugging hhuTOS. 

Last update: 07.04.2025.

## Preparation

### Rust Compiler

For building hhuTOS, a _rust nightly_ toolchain is needed. To install _rust_ use [rustup](https://rustup.rs/). The toolchain `nightly-2025-03-10` is confirmed to work with hhuTOS.
We also need `cargo-make`.

```bash
rustup toolchain install nightly-2025-03-10
rustup override set nightly-2025-03-10
cargo install --no-default-features cargo-make
```

Furthermore, we need to install the _build-essential_ tools, as well as the _Netwide Assembler_ (nasm) for building hhuTOS.
We use _GRUB_ as a bootloader and need _xorriso_ for building a bootable ISO image.
Last but not least, QEMU is recommended to run hhuTOS in a virtual machine and _GDB_ for debugging.

On Ubuntu 24.04 you can install all the above with a single apt command:

```bash
sudo apt install build-essential nasm grub-pc xorriso qemu-system-x86 gdb
```

On macOS, we can install most of the tools via [brew](https://brew.sh/):

```bash
brew install x86_64-elf-binutils nasm xorriso qemu x86_64-elf-gdb
```

Unfortunately the correct GRUB version cannot be installed via brew and needs to be compiled manually:

```bash
brew install x86_64-elf-gcc
git clone git://git.savannah.gnu.org/grub.git && cd grub
./bootstrap
./autogen.sh
./configure --disable-werror TARGET_CC=x86_64-elf-gcc TARGET_OBJCOPY=x86_64-elf-objcopy TARGET_STRIP=x86_64-elf-strip TARGET_NM=x86_64-elf-nm TARGET_RANLIB=x86_64-elf-ranlib --target=x86_64-elf --prefix=$HOME/opt/grub
make -j8
make install
```

As a last step, we need to add GRUB to our `PATH` variable:

```bash
export PATH=$PATH:~/opt/grub/bin
```

## Compiling
For a full build run: 

`cargo make`

## Running

To run the image, use the following command, which will automatically build hhuTOS and run it in QEMU:

`cargo make qemu`

## Debugging 

hhuTOS contains configuration files for VSCode. To use VSCode for Debugging, just install the _Rust_ and _C++_ extensions (_Memory Viewer_ is also recommended) and the debug target `qemu-gdb` should appear in the `Run and Debug` tab in VSCode. Just click the play button to start a debugging session. 

It is also possible to debug hhuTOS via GDB in a terminal. To start QEMU with GDB-server run the following command in a terminal (this should open `qemu` but not boot hhuTOS yet):

```bash
cargo make qemu-gdb
```

Open another terminal and start a GDB debugging session:

```bash
cargo make gdb
```

For more convenient debugging, we recommend starting GDB with the integrated _Terminal UI_ (TUI):

```bash
cargo make gdbt
```
