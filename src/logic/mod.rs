use std::fs;
use std::io::{self, Write};
use crate::models::{Command, CommandError};
pub type Result<T> = std::result::Result<T, CommandError>;
use std::process::Command as StdCommand;
const COMMANDS: &[(&str, &str)] = &[
    ("dir", "Lists the contents of a directory."),
    ("help", "Displays this list of commands."),
    ("vol", "Displays the disk volume label and serial number."),
    ("path", "Displays or sets a search path for executable files."),
    ("tasklist", "Displays a list of currently running processes."),
    ("notepad", "Opens Notepad, a simple text editor."),
    ("echo", "Displays messages, or turns command echoing on or off."),
    ("color", "Sets the default console foreground and background colors."),
    ("ping", "Sends ICMP ECHO_REQUEST packets to network hosts."),
    ("exit", "Exit the Shell")
];

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
        "q" => Command::Exit,
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
        Command::Ping(arg1) => execute_ping(&[arg1]),
        _ => Err(CommandError::NotFound("Command not found\n".to_string()))
    };

    // If there was an error executing the command, print it out
    if let Err(e) = result {
        println!("\nError executing command: {}", e);
    }
}

fn execute_dir(args: &[Option<String>]) -> Result<()> {
    // If args is None, print the current directory
    if args[0].is_none() {
        let output = StdCommand::new("ls")
            .arg("-l")
            .output()
            .expect("Failed to execute command\n");
        let result = String::from_utf8_lossy(&output.stdout);
        println!("{}", result);
    } else {
        println!("Todo!");
    }

    Ok(())
}




fn execute_help() -> Result<()> {
    println!("\nAvailable commands:");
    for &(command, description) in COMMANDS.iter() {
        println!("{:10} - {}", command, description);
    }
    println!();
    Ok(())
}

fn execute_vol() -> Result<()> {

    Ok(())
}

fn execute_path() -> Result<()> {

    Ok(())
}
fn execute_tasklist() -> Result<()> {

    Ok(())
}
fn execute_notepad() -> Result<()> {

    Ok(())
}


// .join() bug (SOLVED)
// The .join() method cannot be used with Vec<&String> or Vec<String> due to trait bounds.
// Solution: Borrow the data as a &str
// When you call .as_ref().map(String::as_str) inside filter_map, you're creating temporary borrows of the string data.
// These borrows exist for the duration of the execute_echo call and do not affect the ownership of the original strings contained within the args array.
fn execute_echo(args: &[Option<String>]) -> Result<()> {
    // Convert Vec<&String> to Vec<&str> to satisfy the trait bounds for .join()
    let echo_str = args
        .iter()
        .filter_map(|arg| arg.as_ref().map(String::as_str)) // Convert &String to &str here
        .collect::<Vec<&str>>() // Now we have Vec<&str>, which can call .join()
        .join(" ");
    println!("{}", echo_str);
    Ok(())
}




fn execute_color(arg: &[Option<String>]) -> Result<()> {
    let color = match arg.first() {
        Some(Some(arg)) => arg,
        Some(None) | None => return Err(CommandError::MissingArguments("color <colorname>\n".to_string()))
    };

    println!("{}", color);
    Ok(())
}



fn execute_ping(args: &[Option<String>]) -> Result<()> {
    let address = match args.first() {
        Some(Some(arg)) => arg,
        Some(None) | None => return Err(CommandError::MissingArguments("ping <address>\n".to_string())),
    };

    let command = StdCommand::new("ping").arg("127.0.0.1").arg("-c").arg("4").output();
    println!("{:?}", command.unwrap());

    Ok(())
        // // Determine the correct argument to limit ping based on the OS
        // #[cfg(target_os = "windows")]
        //     let count_arg = "-n";
        // #[cfg(not(target_os = "windows"))]
        //     let count_arg = "-c";
        //
        // match StdCommand::new("ping").arg(count_arg).arg("4").arg(address).output() {
        //     Ok(output) => match output.status.success() {
        //         true => {
        //             // Command executed successfully, print the stdout of the command
        //             println!("{}", String::from_utf8_lossy(&output.stdout));
        //             Ok(())
        //         },
        //         false => {
        //             // Command execution failed, print the stderr of the command
        //             let error_str = String::from_utf8_lossy(&output.stderr);
        //             eprintln!("{}", error_str); // Write to stderr
        //             Err(CommandError::CommandFailed(format!("Ping to {} failed", address)))
        //         },
        //     },
        //     Err(e) => {
        //         // Error executing the command, write the error to stderr
        //         eprintln!("Failed to execute ping command: {}", e);
        //         Err(CommandError::CommandFailed(format!("Failed to execute ping: {}", e)))
        //     },
        // }


    // match StdCommand::new("ping").arg(address).output() {
    //     Ok(output) => match output.status.success() {
    //         true => {
    //             // Command executed successfully, print the stdout of the command
    //             println!("{}", String::from_utf8_lossy(&output.stdout));
    //             Ok(())
    //         },
    //         false => {
    //             // Command execution failed, print the stderr of the command
    //             let error_str = String::from_utf8_lossy(&output.stderr);
    //             eprintln!("{}", error_str);
    //             Err(CommandError::CommandFailed(format!("Ping to {} failed", address)))
    //         },
    //     },
    //     Err(e) => {
    //         // Error executing the command, write the error to stderr
    //         eprintln!("Failed to execute ping command: {}", e);
    //         Err(CommandError::CommandFailed(format!("Failed to execute ping: {}", e)))
    //     },
    // }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_with_valid_address() {
        let args = vec![Some("127.0.0.1".to_string())];
        let result = execute_ping(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ping_with_invalid_address() {
        let args = vec![Some("256.6.9.6".to_string())]; // Assuming this address is considered invalid
        let result = execute_ping(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_ping_with_missing_arguments() {
        let args: Vec<Option<String>> = vec![]; // No arguments provided
        let result = execute_ping(&args);
        assert!(result.is_err());
        if let Err(CommandError::MissingArguments(msg)) = result {
            assert_eq!(msg, "ping <address>\n");
        } else {
            panic!("Expected MissingArguments error");
        }
    }
}

