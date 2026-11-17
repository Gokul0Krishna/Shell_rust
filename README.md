# Shell Rust

A small interactive command-line shell written in Rust. The program displays the current working directory as a prompt, accepts commands from the terminal, executes them through the operating system, and prints their standard output.

## Features

- Interactive terminal prompt showing the current directory
- External command execution with arguments
- Standard output display for executed commands
- Simple command exit using `BANG` or `bang`
- Built-in ASCII art exit message
- Powered by the [`rprompt`](https://crates.io/crates/rprompt) crate for terminal input

## Requirements

- Rust and Cargo
- A supported operating system and shell command environment

## Getting Started

Clone the project and move into the project directory:

```bash
git clone <repository-url>
cd Shell_rust
```

Build and run the shell:

```bash
cargo run
```

## Usage

After starting the application, enter commands at the directory-based prompt:

```text
/home/gokul/Desktop/code/Shell_rust>>ls -l
```

The shell runs the command and prints its output. For example, `pwd`, `ls -l`, or `echo Hello` can be entered as text.

To stop the shell, enter:

```text
BANG
```

The lowercase form also works:

```text
bang
```

## Project Structure

```text
Shell_rust/
├── Cargo.toml       # Package metadata and dependencies
├── README.md        # Project documentation
└── src/
    └── main.rs      # Shell prompt and command execution logic
```

## How It Works

1. `main()` starts the shell loop in interactive mode.
2. The prompt displays the current working directory.
3. `rprompt` reads input entered by the user.
4. `parse_input()` separates the command from its arguments.
5. `std::process::Command` executes the command.
6. Standard output is converted to text and printed to the terminal.

## Current Limitations

- Commands are passed to the operating system rather than interpreted as built-in shell features.
- Pipes, redirection, environment-variable expansion, and command chaining are not implemented.
- The shell does not currently handle an empty input line safely.
- Command errors and standard error output are not fully presented.

## Dependencies

- [rprompt](https://crates.io/crates/rprompt) — terminal prompt and input handling

## License

Add your preferred license before publishing or distributing this project.

