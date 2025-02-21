# Rust Password Generator

A simple CLI password generator written in Rust that creates a random alphanumeric password of a given length (default is 12) and copies it straight to your clipboard. Because who has time to type a password when your computer can do it for you?

## Features

- **Random Passwords:** Generates a password with a mix of numbers and letters.
- **Customizable Length:** Provide your desired length as a command-line argument, or use the default.
- **Clipboard Integration:** Automatically copies the generated password to your clipboard for easy pasting.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (and Cargo)
- Linux with X11 (if using `cli_clipboard::x11_clipboard`)

## Usage

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/yourusername/rust-password-generator.git
   cd rust-password-generator
   ```

2. **Build the Program:**

   ```bash
   cargo build --release
   ```

3. **Run the Program:**

   - **Default Password (12 characters):**

     ```bash
     cargo run --release
     ```

   - **Custom Length (e.g., 16 characters):**

     ```bash
     cargo run --release 16
     ```
     
   - **Custom Length (e.g., 16 characters):**

     ```bash
     cargo run --release --no-symbols
     ```

   - **Custom Length (e.g., 16 characters) && without special symbols:**

     ```bash
     cargo run --release 16 --no-symbols
     ```
After running, the generated password is printed to the console and copied to your clipboard. Now you can confidently use that password without ever typing it!

## Notes

- The program uses the [`rand`](https://docs.rs/rand) crate for random generation.
- Clipboard handling is managed by the [`cli_clipboard`](https://crates.io/crates/cli_clipboard) crate.
- If you encounter any issues with clipboard integration on your system, check out the crate documentation for troubleshooting tips.
