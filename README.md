# ⚙️ Basic_os: A Minimal Rust Operating System Kernel

<div align="center">

<!-- TODO: Add project logo/icon if available -->

[![GitHub stars](https://img.shields.io/github/stars/BSZKaneki/Basic_os?style=for-the-badge)](https://github.com/BSZKaneki/Basic_os/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/BSZKaneki/Basic_os?style=for-the-badge)](https://github.com/BSZKaneki/Basic_os/network)
[![GitHub issues](https://img.shields.io/github/issues/BSZKaneki/Basic_os?style=for-the-badge)](https://github.com/BSZKaneki/Basic_os/issues)
[![GitHub license](https://img.shields.io/github/license/BSZKaneki/Basic_os?style=for-the-badge)](LICENSE)

**A foundational operating system kernel, meticulously crafted from the ground up using Rust.**

</div>

## 📖 Overview

Basic_os is an experimental and educational project focused on building a minimal operating system kernel using the Rust programming language. It leverages Rust's powerful safety guarantees and low-level capabilities to create a robust and reliable foundation for an OS.

This project serves as an excellent starting point for anyone interested in bare-metal programming, understanding the internals of an operating system, or exploring Rust's potential beyond application development. It focuses on the essential components required to boot a custom kernel and interact directly with hardware.

## ✨ Features

-   **Bare-metal Rust Development**: Core kernel logic written entirely in Rust, interacting directly with hardware without an underlying OS.
-   **Custom Target Configuration**: Utilizes a custom target specification (`rust_os.json`) for precise control over the compilation environment, essential for OS development.
-   **Cargo-managed Project**: Leverages Cargo for robust dependency management, building, and testing of the kernel.
-   **Minimalist Design**: Aims for a lean and efficient kernel, focusing on fundamental OS primitives.
-   **Modular Structure**: Organized codebase to facilitate understanding and expansion of OS components.

## 🛠️ Tech Stack

**Core Language:**
![Rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=white)

**Build System & Tooling:**
![Cargo](https://img.shields.io/badge/Cargo-black?style=for-the-badge&logo=rust&logoColor=white)
![Rustup](https://img.shields.io/badge/Rustup-black?style=for-the-badge&logo=rust&logoColor=white)

**OS Development Tools (External):**
![QEMU](https://img.shields.io/badge/QEMU-black?style=for-the-badge&logo=qemu&logoColor=white)

## 🚀 Quick Start

To get Basic_os up and running in an emulated environment, follow these steps.

### Prerequisites

Ensure you have the following installed on your system:

-   **Rust Toolchain**: Install `rustup` to manage Rust versions.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
-   **Specific Rust Toolchain**: This project requires a specific Rust toolchain version and components as defined in `rust-toolchain.toml`. Install it:
    ```bash
    rustup override set $(cat rust-toolchain.toml | grep -o 'channel = "[^"]*"' | cut -d'"' -f2)
    rustup target add x86_64-unknown-none # Add the bare-metal target for building
    ```
-   **`cargo-binutils`**: Essential for working with custom targets and generating bootable images.
    ```bash
    cargo install cargo-binutils
    rustup component add llvm-tools-preview
    ```
-   **QEMU**: A generic and open-source machine emulator and virtualizer, used to run the compiled OS kernel.
    ```bash
    # On Debian/Ubuntu
    sudo apt install qemu-system-x86

    # On Fedora
    sudo dnf install qemu-system-x86

    # On macOS with Homebrew
    brew install qemu
    ```

### Installation

1.  **Clone the repository**
    ```bash
    git clone https://github.com/BSZKaneki/Basic_os.git
    cd Basic_os
    ```

2.  **Add custom target specification**
    The project uses a custom target named `rust_os.json`. You need to tell Cargo about this target.
    ```bash
    rustup target add --json rust_os.json
    ```
    *Note: This command is a placeholder; typically, you'd place the `rust_os.json` file in a `.cargo/config.toml` defined build path, or a tool like `bootloader` handles this automatically.*

3.  **Build the kernel**
    Compile the kernel for the custom `rust_os` target.
    ```bash
    cargo build --target rust_os.json
    ```
    This command will generate the kernel executable in `target/rust_os/debug/basic_os`.
    For a release build:
    ```bash
    cargo build --release --target rust_os.json
    ```

4.  **Create a bootable image**
    This step typically involves using `bootimage` (a common Rust OS dev crate) or `cargo-binutils` to link the kernel with a bootloader. Assuming the project is set up to use `bootimage` implicitly (as is common for `Basic_os`-style projects):
    ```bash
    cargo bootimage --target rust_os.json
    ```
    If `bootimage` is not integrated, you might need manual steps involving `objcopy` from `cargo-binutils`:
    ```bash
    # Example using cargo-objcopy, actual command might vary based on bootloader
    cargo objcopy -- -O binary target/rust_os/debug/basic_os target/basic_os.bin
    ```
    This will produce a bootable image (e.g., `bootimage-basic_os.bin`) in your `target/rust_os/debug` or `target/x86_64-rust_os/debug` directory.

5.  **Run in QEMU**
    Execute the bootable image using QEMU:
    ```bash
    qemu-system-x86_64 -drive format=raw,file=target/rust_os/debug/bootimage-basic_os # Or wherever the bootimage is located
    ```
    This will launch a QEMU window, where you should see the output from your kernel.

## 📁 Project Structure

```
Basic_os/
├── .cargo/             # Cargo configuration directory
├── .gitignore          # Git ignore file
├── Cargo.lock          # Locked dependencies for Cargo
├── Cargo.toml          # Project manifest and dependencies
├── rust-toolchain.toml # Specifies the Rust toolchain version
├── rust_os.json        # Custom target specification for the OS
├── src/                # Kernel source code
│   └── main.rs         # Main entry point of the kernel
└── tests/              # Integration and unit tests
    └── ...             # Test files
```

## 🔧 Development

### Building
To build the kernel for development:
```bash
cargo build --target rust_os.json
```
For an optimized release build:
```bash
cargo build --release --target rust_os.json
```

### Running in Emulator
After building, run the generated bootable image with QEMU:
```bash
qemu-system-x86_64 -drive format=raw,file=target/rust_os/debug/bootimage-basic_os
```
Replace `target/rust_os/debug/bootimage-basic_os` with the actual path to your bootable image.

## 🧪 Testing

This project includes tests to ensure the correctness of kernel components.

```bash
# Run all tests
cargo test
```
*Note: Due to the nature of OS development, many kernel tests might require specific execution environments or custom test runners not directly invoked by `cargo test`. The `tests/` directory likely contains integration tests or specific unit tests that can run in a hosted environment.*

