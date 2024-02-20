use std::fs;
use crate::models::{Command, CommandError};
use std::process::Command as StdCommand;


pub fn parse_command(input: &str) -> Command {
    // Split the user input into pieces based on whitespace
    let parts: Vec<&str> = input.split_whitespace().collect();

    // Get the first piece, which should be the command
    let command = parts.first().cloned().unwrap_or("");

    // If additional arguments exist, turn them into a Some(String)
    // If not, .get() returns None
    let arg1 = parts.get(1).map(|s| s.to_string());
    let arg2 = parts.get(2).map(|s| s.to_string());
    let arg3 = parts.get(3).map(|s| s.to_string());
    let arg4 = parts.get(4).map(|s| s.to_string());


    // Based on the command the user inputted, return the correct state
    match command {
        "dir" => Command::Dir(arg1, arg2, arg3, arg4),
        "help" => Command::Help,
        "vol" => Command::Vol,
        "path" => Command::Path,
        "tasklist" => Command::TaskList,
        "notepad" => Command::Notepad,
        "echo" => Command::Echo(arg1, arg2, arg3, arg4),
        "color" => Command::Color(arg1),
        "ping" => Command::Ping(arg1),
        "exit" => Command::Exit,
        "quit" => Command::Exit,
        _ => Command::Unknown,
    }
}


pub fn execute_command(command: Command)  {
    let result = match command {
        Command::Dir(arg1, arg2, arg3, arg4) => execute_dir(&[arg1, arg2, arg3, arg4]),
        Command::Help => execute_help(),
        Command::Vol => execute_vol(),
        Command::Path => execute_path(),
        Command::TaskList => execute_tasklist(),
        Command::Notepad => execute_notepad(),
        Command::Echo(arg1, arg2, arg3, arg4) => execute_echo(&[arg1, arg2, arg3, arg4]),
        Command::Color(arg1) => execute_color(&[arg1]),
        _ => Err(CommandError::NotFound("Command not found".to_string()))
    };

    if let Err(e) = result {
        println!("Error executing command: {:?}", e);
    }
}

















fn execute_dir(args: &[Option<String>]) -> Result<(), CommandError> {
    // If args is None, print the current directory
    if args[0].is_none() {
        let output = StdCommand::new("ls")
            .arg("-l")
            .output()
            .expect("Failed to execute command");
        let result = String::from_utf8_lossy(&output.stdout);
        println!("{}", result);
    } else {
        println!("Todo!");
    }

    Ok(())
}




fn execute_help() -> Result<(), CommandError> {

    Ok(())
}

fn execute_vol() -> Result<(), CommandError> {

    Ok(())
}

fn execute_path() -> Result<(), CommandError> {

    Ok(())
}
fn execute_tasklist() -> Result<(), CommandError> {

    Ok(())
}
fn execute_notepad() -> Result<(), CommandError> {

    Ok(())
}


// .join() bug (SOLVED)
// The .join() method cannot be used with Vec<&String> or Vec<String> due to trait bounds.
// Solution: Borrow the data as a &str
// When you call .as_ref().map(String::as_str) inside filter_map, you're creating temporary borrows of the string data.
// These borrows exist for the duration of the execute_echo call and do not affect the ownership of the original strings contained within the args array.
fn execute_echo(args: &[Option<String>]) -> Result<(), CommandError> {
    // Convert Vec<&String> to Vec<&str> to satisfy the trait bounds for .join()
    let echo_str = args
        .iter()
        .filter_map(|arg| arg.as_ref().map(String::as_str)) // Convert &String to &str here
        .collect::<Vec<&str>>() // Now we have Vec<&str>, which can call .join()
        .join(" ");
    println!("{}", echo_str);
    Ok(())
}




fn execute_color(arg: &[Option<String>]) -> Result<(), CommandError> {

    Ok(())
}



fn execute_ping(args: &[Option<String>]) -> Result<(), CommandError> {
    if let Some(address) = args.first() {
        println!("Pinging {}... Success!", address.as_ref().unwrap());
        Ok(())
    } else {
        Err(CommandError::InvalidArgument("Invalid ping address".to_string()))
    }
}