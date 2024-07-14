# Save Windows Status


### Project Purpose
    This project is aimed at studying the Rust programming language.



Save Windows Status is a GNOME application that saves and restores the state of your open windows.

## Features

- Save the state of open windows
- Restore the state of open windows
- Option to clear saved state
- Simple GTK interface

## Installation

### Prerequisites

Ensure you have the following dependencies installed:

- `libgtk-3-0`
- `wmctrl`

On Ubuntu/Debian, you can install these dependencies with:

```sh
sudo apt-get install libgtk-3-0 wmctrl
```

- [Rust](https://www.rust-lang.org/)
- [Cargo](https://github.com/rust-lang/cargo#compiling/) 

Building from Source
Clone the repository:

```sh
git clone https://github.com/gustavoguarda/save-windows-status.git
cd save-windows-status
```

Build the application:

```sh
cargo build --release
cargo run
```

## Building a Debian Package

You can build a Debian package for `save-windows-status` using `cargo deb`.

1. Install `cargo deb`:

    ```sh
    cargo install cargo-deb
    ```

2. Build the Debian package:

    ```sh
    cargo deb
    ```

This will create a `.deb` package in the `target/debian` directory. You can install the package with:

```sh
sudo dpkg -i target/debian/save-windows-status_0.1.0_amd64.deb
```

## GNOME Integration

*This section is under development.*

Future updates will include detailed instructions on how to integrate `save-windows-status` with GNOME for seamless window state saving and restoration.


Author
Gustavo Guarda - gustavoguarda@gmail.com