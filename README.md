# PSWDGEN

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Repository](https://img.shields.io/badge/repo-ArutyunyanA%2FPSWDGEN-blue)](https://github.com/ArutyunyanA/PSWDGEN)

**A fast, reliable CLI utility for generating secure passwords and saving them to files.**

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Configuration](#configuration) • [Contributing](#contributing)

</div>

---

## Overview

PSWDGEN is a lightweight command-line tool written in **Rust** that simplifies password generation and file management. Whether you need a single strong password or batch generation, PSWDGEN delivers with speed and reliability.

### Why PSWDGEN?

- ⚡ **Blazing Fast** — Compiled Rust ensures optimal performance
- 🔒 **Secure** — Cryptographically secure random password generation
- 💾 **File Integration** — Seamlessly save passwords to `.txt` files
- 🎨 **User Friendly** — Clean CLI interface with visual feedback
- 🔧 **Configurable** — Customize password complexity and length
- 📦 **Zero Dependencies Bloat** — Minimal, focused dependencies

---

## Features

✨ **Core Capabilities:**

- Generate passwords with custom parameters
- Support for various character sets (uppercase, lowercase, numbers, symbols)
- Save single or multiple passwords to text files
- Load configuration from `.env` file
- Clean, intuitive command-line interface with ASCII art branding
- Cross-platform compatibility (Linux, macOS, Windows)

---

## Installation

### From Source

**Prerequisites:**
- Rust 1.70 or later ([Install Rust](https://rustup.rs/))
- Cargo (included with Rust)

**Steps:**

1. Clone the repository:
```bash
git clone https://github.com/ArutyunyanA/PSWDGEN.git
cd PSWDGEN
```
```bash
cargo build --release
```
### Passwords generator CLI utility for command prompt generating passwords and saving it to the .txt file.


2. Installation.

```bash
cargo install --path .
```

3. Usage.


```bash

MacBook-Air-Witcher:~ macintosh$ pswdgen -h
Usage:

    pswdgen <length> <count> [options]

Arguments:

    <length>               Password length (number of characters)
    <count>                How many passwords to generate

Options:

    -h, --help             Show this help menu
    -v, --version          Display version information

Examples:

    pswdgen 16 5
    pswdgen 24 10 > passwords.txt

Description:

    Generates random passwords using cryptographic RNG (OsRng).
    Each password may optionally be labeled interactively.


MacBook-Air-Witcher:~ macintosh$ pswdgen 16 2

██████╗ ███████╗██╗    ██╗██████╗  ██████╗ ███████╗███╗   ██╗
██╔══██╗██╔════╝██║    ██║██╔══██╗██╔════╝ ██╔════╝████╗  ██║
██████╔╝███████╗██║ █╗ ██║██║  ██║██║  ███╗█████╗  ██╔██╗ ██║
██╔═══╝ ╚════██║██║███╗██║██║  ██║██║   ██║██╔══╝  ██║╚██╗██║
██║     ███████║╚███╔███╔╝██████╔╝╚██████╔╝███████╗██║ ╚████║
╚═╝     ╚══════╝ ╚══╝╚══╝ ╚═════╝  ╚═════╝ ╚══════╝╚═╝  ╚═══╝
                                                             

Would you like to specify the passwords name?
Answers: 'yes' or 'no'
yes
Enter the name of resource or website: 
HackerOne
Would you like to specify the passwords name?
Answers: 'yes' or 'no'
yes   
Enter the name of resource or website: 
Google

Passwords saved to passwords.txt

MacBook-Air-Witcher:~ macintosh$ cat passwords.txt
 
Google: Gl!Y4=h%Bp14V>37
HackerOne: f91qhQxt=$jSC6zR
```

Architecture
Project Structure
```code
PSWDGEN/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── generator.rs      # Password generation logic
│   ├── file_handler.rs   # File I/O operations
│   └── config.rs         # Configuration management
├── Cargo.toml            # Project manifest
├── .env.example          # Example environment config
└── README.md             # This file
```
Key Dependencies
Crate	Purpose	Version
rand	Cryptographic randomness	0.8+
dotenv	Environment variable loading	0.15+
figlet-rs	ASCII art branding	0.1+

Security Considerations

⚠️ Important:

    Passwords are generated using rand::rngs::OsRng for cryptographic security
    Ensure proper file permissions on generated password files
    Consider deleting temporary password files after use
    Do not commit password files to version control

Contributing

Contributions are welcome! Here's how to get started:

    Fork the repository
    Create a feature branch (git checkout -b feature/amazing-feature)
    Commit your changes (git commit -m 'Add amazing feature')
    Push to the branch (git push origin feature/amazing-feature)
    Open a Pull Request

Guidelines

    Follow Rust naming conventions
    Add tests for new functionality
    Update documentation
    Keep commits atomic and meaningful
    Ensure cargo clippy and cargo fmt pass

License

This project is licensed under the MIT License — see the LICENSE file for details.

TL;DR: You can use, modify, and distribute this software freely with proper attribution.
Support

    📖 Documentation: Check README.md and code comments
    🐛 Report Issues: GitHub Issues
    💬 Discussions: GitHub Discussions

Author

Arutyunyan Artem

    📧 Email: arutyunyan_av@icloud.com
    🔗 GitHub: @ArutyunyanA

Acknowledgments

    Built with Rust 🦀
    Inspired by best practices in CLI tool design
    Thanks to the Rust community for excellent tools and documentation

Made with ❤️ in Rust
















