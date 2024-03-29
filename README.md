# Interactive-Shell
A Rudimentary Shell Command Line Interpreter designed to offer basic functionalities for system interaction and process management. This shell supports a variety of commands for managing files, processes, and system information.

## Features

- Cross-platform compatibility, ensuring the shell can be utilized across different operating systems.

## Available Commands

Below is a list of commands available in the Interactive-Shell, along with their descriptions:

- `dir` - Lists the contents of a directory.
- `help` - Displays the list of available commands.
- `vol` - Shows the disk volume label and serial number.
- `path` - Displays or sets the search path for executable files.
- `tasklist` - Lists currently running processes.
- `notepad` - Opens Notepad, a simple text editor.
- `echo` - Prints messages, or toggles command echoing on or off.
- `color` - Configures the default console foreground and background colors.
- `ping` - Initiates ICMP ECHO_REQUEST packets to network hosts.
- `exit` - Exits the Shell.

## Getting Started

### Prerequisites

Ensure you have the Rust compiler (`rustc`) and Cargo (Rust's package manager and build system) installed on your system to compile and run the Interactive-Shell. The shell is developed using Rust, which ensures safety and performance.

### Installation

1. **Download Rustc Compiler and Cargo**:
    - **Via Official Website**: Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to download and install the Rust compiler and Cargo.
    - **Via Command Line** (for Unix-like operating systems such as Linux and macOS):
      ```sh
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
      This command downloads and executes the rustup script, which installs the stable version of Rust. If you're using Windows, you can use the rustup-init.exe from the Rust website or use Windows Subsystem for Linux (WSL) to run the command.

2. **Clone the Repository**: Clone the Interactive-Shell repository to your local machine using Git:
   ```sh
   git clone https://github.com/GMoney999/Interactive-Shell.git

3. **Build and Run**: Navigate to the root directory of the cloned repository and run the following command to compile and launch the shell:
    ```sh
    cargo run
    ```
   
---

## Command API 

### 'dir'
Lists the contents of the current working directory. Equivalent to `dir` in Windows and `ls` in Linux.

---

### 'help'
Get information on all available commands.

---

### 'vol'



--- 

### 'path'
Displays or sets the search path for executable files

---

### 'tasklist'
Lists currently running processes.

--- 

### 'notepad'
Opens Notepad, a simple text editor.

--- 

### 'echo'
Prints messages, or toggles command echoing on or off.

---

### 'color'
Configures the default console foreground and background colors.

### 'ping'
Initiates ICMP ECHO_REQUEST packets to network hosts

---

### 'quit'
Exit the shell

Can also do `q` or `exit`

---




