use std::process;
use errno::{Errno, Error};
use std::io::{self, BufRead, Write};
use fork::{fork, Fork};
use std::env;
use std::ptr;
use libc::execvp;

fn c_exec(command: &str, args: Vec<&str>) {
    let mut res;
    let mut args_ptr: Vec<*const u8> = args.iter().map(|s| s.as_ptr()).collect();
    args_ptr.push(ptr::null());

    unsafe {
        res = execvp(command.as_ptr(), args_ptr.as_ptr());
    };

    Err(Error::Sys(Errno::last()))
}

fn moonsh_launch(args: Vec<&str>) -> i32 {
    let mut child;
    let mut wpid;
    let mut status;

    match fork() {
        Ok(Fork::Parent(child)) => {
            println!("Parent executing, new child has id: {}", child);
            loop {
                //let wpid = waitpid(child, status.as_ptr(), WUNTRACED);
                break;
            }
        }

        Ok(Fork::Child) => {
            println!("Forked to child");
            c_exec(args[0], args[1..])
        }

        Err(_) => {
            println!("Failed to fork to child process.");
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

        // Parse command into args for syscall
        let args: Vec<&str> = line.split(' ').collect();

        status = moonsh_launch(args);
        
        if status == 0 {
            break 0;
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
