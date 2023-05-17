**Shell using the Rust Programming Language**


**Program Structure:**

    SimpleShell Struct: Represents the struct itself that contains methods for dealing with commands

    Parse_command method: Tokenizes the input command into individual tokens and splits them by whitespace and stores them in a vector

    exec_command method: Executes the command by creating a child process using .spawn() and the command struct from the procces module. Calls the "dir" command to execute in a windows command prompt.

    is_quit() method: used to check if the quit command is enterd and terminates the shell

    main() method: entry point for the program which creates and instance of the SimpleShell struct and loops to prompt the user for an input.


**How to run:**
    Compile and build the program with a rust compiler
    
    run the program

    prompted with 'tsh>  ' indicating that you are in the shell and ready to accept commands.

    the program will process the commands

**Design Decisions:**

    Used similar logic to the the simple shell project from class implemented in cpp. 

    Had trouble trying to use "ls" command while running the program so had to do a workaround by using the windows-specific command:
        
        cmd.arg("/C").arg("dir").args(&argv[1..]);


**Dependencies:**

    used 'std::io' for user input and output
    used 'std::process' as a way to spawn child processes and executing commands


**Slideshow**
    https://docs.google.com/presentation/d/1qmgmoxNaxIO7AqkDMNJPegQifRwqydhhJ61okD4YbD4/edit?usp=sharing

**Video**
    https://drive.google.com/file/d/1DBibiIIKxW7earnGJBcKBTWU5vTtXpkI/view?usp=share_link

    **might have to download the file in order to play the video