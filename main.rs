
use std::io::{self, Write};
use std::process::{Command, exit};

struct SimpleShell;

impl SimpleShell {

    //function parses command into tokens
    fn parse_command(&self, cmd: &str, cmd_tokens: &mut Vec<String>) {
        for token in cmd.split_whitespace() {
            //this splits the command string by whitespace and puts the "token" into the string vector
            cmd_tokens.push(token.to_string());
        }
    }

    fn exec_command(&self, argv: &[String]) {
        // let mut cmd = Command::new(&argv[0]);
        //create a new instance of command which is a process builder that gives a path to be executed
        let mut cmd = Command::new("cmd");

        //set the arguments for the command. note this may only work for windows
        cmd.arg("/C").arg("dir").args(&argv[1..]);

        // loop through elements and att to cmd struct that we created
        for arg in &argv[1..] {
            cmd.arg(arg);
        }

        match cmd.spawn() {
            //This is how we spawn a child process for the command
            Ok(mut child) => {
                let _ = child.wait();
            }
            Err(err) => {
                //throw error if command failed
                eprintln!("Command failed: {}", err);
                exit(1);
            }
        }
    }

    //checks for command quit that terminates the shell
    fn is_quit(&self, cmd: &str) -> bool {
        cmd.trim() == "quit"
    }
}

fn main() {
    let mut cmd = String::new();
    let mut cmd_tokens = Vec::new();
    let shell = SimpleShell;

    //user input
    print!("tsh> ");
    io::stdout().flush().unwrap();

    //this loops if the cmd is empty in order to prompt the user again
    while let Ok(_) = io::stdin().read_line(&mut cmd) {
        if cmd.trim().is_empty() {
            print!("tsh> ");
            io::stdout().flush().unwrap();
            continue;
        }

        //if the user wants to quit
        if shell.is_quit(&cmd) {
            exit(0);
        }


        //parse the commands into tokens and execute command
        shell.parse_command(&cmd, &mut cmd_tokens);
        shell.exec_command(&cmd_tokens);

        cmd.clear();
        cmd_tokens.clear();

        print!("tsh> ");
        io::stdout().flush().unwrap();
    }

    println!();
    exit(0);
}