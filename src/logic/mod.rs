use std::fs;
use std::io::{self, Write};
use std::env;
use std::path::{Path};
use crossterm::{
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
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
        "path" => Command::Path(arg1, arg2),
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
        Command::Path(arg1, arg2) => execute_path(&[arg1, arg2]),
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
    let current_dir= env::current_dir().map_err(CommandError::IOError)?;
    println!("\nContents of {:?}", current_dir);

    let entries = fs::read_dir(current_dir).map_err(CommandError::IOError)?;

    for entry in entries {
        let entry = entry.map_err(CommandError::IOError)?;
        let path = entry.path();
        let metadata = fs::metadata(&path).map_err(CommandError::IOError)?;
        let file_type = if metadata.is_dir() { "Directory" } else { "File" };
        println!("{:?} - {}", path.file_name().unwrap(), file_type);
    }
    println!();
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

    #[cfg(target_os = "windows")]
    {
        // Windows: Use 'vol' command for the current drive
        let output = StdCommand::new("cmd")
            .args(&["/C", "vol"])
            .output()
            .expect("Failed to execute process");
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("\nVolume Info (Windows): {}\n", output_str);
    }

    #[cfg(not(target_os = "windows"))]
    {
        // macOS and Linux: Display filesystem type and available space
        let current_dir = env::current_dir().map_err(CommandError::IOError)?;
        let stat = nix::sys::statvfs::statvfs(current_dir.as_path())?;
        let fs_type = stat.filesystem_id();
        // This operation is valid as both operands are u64
        let available_space = stat.blocks_free() as u64 * stat.block_size() as u64;
        println!("\nFilesystem Type: {:?}, Available Space: {} bytes\n", fs_type, available_space);
    }

    Ok(())
}

fn execute_path(args: &[Option<String>]) -> Result<()> {
    let first_arg = args.first().and_then(|opt| opt.as_ref());

    match first_arg {
        None => {
            // Default path behavior: display the current PATH
            if let Ok(path) = env::var("PATH") {
                println!("Current PATH: {}", path);
            } else {
                println!("PATH variable is not set.");
            }
        },
        Some(arg) if arg == "clear" => {
            // Clear the path
            env::set_var("PATH", "");
            println!("PATH has been cleared.");
        },
        Some(arg) if arg == "set" => {
            // Look for second argument. If it is a path, set it.
            match args.get(1).and_then(|opt| opt.as_ref()) {
                Some(path) => {
                    env::set_var("PATH", path);
                    println!("PATH has been set to: {}", path);
                },
                None => {
                    // If the 'set' argument is not followed by a path, throw an error.
                    return Err(CommandError::MissingArguments("\nCorrect usage: path set <PATH>\n".to_string()));
                }
            }
        },
        Some(_) => {
            // If the first argument is anything other than 'set' or 'clear', throw an error.
            return Err(CommandError::InvalidArgument("\nCorrect usage: 'path' OR 'path clear' OR 'path set <PATH>'\n".to_string()));
        }
    }

    Ok(())
}




fn execute_tasklist() -> Result<()> {

    Ok(())
}
fn execute_notepad() -> Result<()> {
    #[cfg(target_os = "windows")]
    match StdCommand::new("notepad.exe").output() {
        Ok(_) => {
            println!("Notepad opened!");
        },
        Err(e) => {
            return Err(CommandError::CommandFailed(format!("Failed to open Notepad: {}", e)));
        }
    };

    #[cfg(not(target_os = "windows"))]
    println!("Notepad is not available on this operating system.");

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



fn parse_color(color: &str) -> Option<Color> {
    match color.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "blue" => Some(Color::Blue),
        "white" => Some(Color::White),
        "yellow" => Some(Color::Yellow),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "grey" => Some(Color::Grey),
        // Add more colors as needed
        _ => None,
    }
}

fn execute_color(args: &[Option<String>]) -> Result<()> {
    let color_arg = match args.first().and_then(|opt| opt.as_ref()) {
        Some(arg) => arg,
        None => return Err(CommandError::MissingArguments("\nCorrect usage: color text=<text color> or color background=<background color>\n".to_string())),
    };

    let mut stdout = io::stdout();

    // Directly parse the argument string for settings
    let settings = color_arg.split_whitespace();

    for setting in settings {
        let parts: Vec<&str> = setting.split('=').collect();
        if parts.len() == 2 {
            let target = parts[0];
            let color = parts[1];

            match target {
                "text" => {
                    if let Some(c) = parse_color(color) {
                        execute!(stdout, SetForegroundColor(c))
                            .map_err(CommandError::IOError)?;
                        execute!(stdout, Clear(ClearType::All)).map_err(CommandError::IOError)?;
                        stdout.flush().map_err(CommandError::IOError)?;
                    }
                },
                "background" => {
                    if let Some(c) = parse_color(color) {
                        execute!(stdout, SetBackgroundColor(c))
                            .map_err(CommandError::IOError)?;
                        execute!(stdout, Clear(ClearType::All)).map_err(CommandError::IOError)?;
                        stdout.flush().map_err(CommandError::IOError)?;
                    }
                },
                _ => println!("Invalid argument: {}", setting),
            }
        } else {
            return Err(CommandError::InvalidArgument("\nCorrect usage: color text=<text color> or color background=<background color>\n".to_string()));
        }
    }




    Ok(())
}




fn execute_ping(args: &[Option<String>]) -> Result<()> {
    let address = match args.first() {
        Some(Some(arg)) => arg,
        Some(None) | None => return Err(CommandError::MissingArguments("\nCorrect usage: ping <address>\n".to_string())),
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
