use std::env::{self};
use std::path::Path;
use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        let current_dir = env::current_dir().unwrap();
        // print!("~{} > ", current_dir.display());
        print!("[Phantomleak] [~{}]", current_dir.display());
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(_command) = commands.next() {

            let mut parts = input.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let mut new_dir = args.peekable().peek().map_or("/", |x| *x);
                    
                    // Allow '~' to act as root
                    if new_dir == "~" {
                        new_dir = "/"
                    }

                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    previous_command = None;
                },
                "exit" => return,
                command => {
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command).args(args).stdin(stdin).stdout(stdout).spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // Block until the final command has finished
            let _ = final_command.wait();
        }
    }
}
