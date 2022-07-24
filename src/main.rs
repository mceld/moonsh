use std::process;
use std::io::{self, BufRead, Write};
use fork::{fork, Fork};
use std::ffi::CString;
use nix::unistd::execvp;
use termination::{EXIT_FAILURE, EXIT_SUCCESS};
use sysinfo::SystemExt;
use sysinfo::ProcessExt;

fn moonsh_launch(command: &str, args: Vec<&str>) -> i32 {
    let mut system = sysinfo::System::new();
    // Fork to a child process and run the given command with args
    //  TODO maybe this needs rewritten with sysinfo ProcessExt and other sysinfo utils instead of
    //  fork
    match fork() {
        Ok(Fork::Parent(child)) => {
            // loop and while child process with id 'child' has not exited, continue to loop
            loop {
                system.refresh_all();
                match system.get_process(child) {
                    Some(proc) => {
                        match &proc.status {
                            Some(stat) => {
                                match stat {
                                    sysinfo::ProcessStatus::Zombie | sysinfo::ProcessStatus::Dead => {
                                        proc.kill(sysinfo::Signal::Kill);
                                        break EXIT_FAILURE;
                                    }
                                    _ => {}
                                }
                            }
                            None => {
                                // Should we panic! ?
                                break EXIT_FAILURE;
                            }
                        }
                    } // continue
                    None => {
                        break EXIT_FAILURE;
                    }
                }
            }
        }
        Ok(Fork::Child) => {
            // This conversion can probably be more robust
            let c_command: CString = CString::new(command).expect("Could not convert command string to CString.");
            let c_args: Vec<CString> = args.iter().map(|arg| CString::new(arg.to_owned()).expect("Could not convert an arg.")).collect();
            
            match execvp(&c_command, c_args.as_slice()) {
                Err(e) => {
                    println!("{}: {}", command, e);
                    process::exit(EXIT_FAILURE)
                }
                _ => {
                    process::exit(EXIT_FAILURE)
                }
            }
        }
        Err(_) => {
            println!("Could not fork.");
            EXIT_SUCCESS
        }
    }
}

fn moonsh_read_line() -> io::Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buf)?;

    Ok(buf)
}

fn moonsh_loop(prompt: &str) -> i32 {
    let mut status: i32;
    
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Could not flush stdout.");

        // Read line and check for read errors
        let line;
        match moonsh_read_line() {
            Ok(l) => line = l,
            Err(e) => {
                println!("Error reading line from stdin: {}", e);
                return 1;
            }
        }

        // Parse command into args for sys call
        let mut args: Vec<&str> = line.split(' ').collect();

        // Trim leading and trailing whitespace
        args = args.iter().map(|arg| arg.trim()).collect();

        status = moonsh_launch(args[0], args);

        if status == 0 {
            break EXIT_SUCCESS;
        }
    }
}


fn main() {
    // config
    let prompt: &str = "moonsh$ ";
    
    // loop
    let code: i32 = moonsh_loop(prompt);

    // clean up

    process::exit(code)
}

// Status:
// Basic REPL working with stubbed execution functions
// Will need to read up on std::unix::execvp or other methods
// Need to find programs in path variable (use your own approach if needed)
//
// copying execvp crate code for executing commands, need to know how perror works -> follow shell
// tutorial
