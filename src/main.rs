#![allow(dead_code, unused_imports, unused_variables)]
mod models;
use models::{Command, CommandError};
mod logic;
use logic::{execute_command, parse_command};
use std::io::{self, Read, Write};
use std::{fs};
use std::thread;

fn main() {
    std::process::exit(real_main())
}

fn real_main() -> i32 {
    println!("\nWelcome to MyShell!\n\n");

    // Start an infinite loop
    loop {
        print!("==> ");
        // Flush stdin
        io::stdout().flush().unwrap();

        // Create string to hold user input
        let mut input = String::new();

        // Read user input from stdin
        // If successful, parse the input
        // If failed, print error
        if io::stdin().read_line(&mut input).is_ok() {
            // Remove leading and trailing whitespaces
            let input = input.trim();

            // Parse user input
            let command = parse_command(input);

            // If the command found was anything else besides exit, execute it
            match command {
                Command::Exit => {
                    println!("\nThank you for using MyShell!\n");
                    return 0;
                },
                Command::Unknown => {
                    println!("Invalid command: {input}");
                },
                _ => {
                    // Spawn a new thread to execute the command in the new thread
                    let handle = thread::spawn(move || execute_command(command));

                    // Wait for thread to finish
                    handle.join().unwrap();
                }
            }
        } else {
            eprintln!("Failed to read input line");
            return 1;
        }
    }
}

















